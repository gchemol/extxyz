// [[file:../../extxyz.note::3812ce95][3812ce95]]
use super::{label, recognize_sci_float};
use crate::{RawAtom, RawAtoms};

use winnow::ascii::{space0, space1};
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::separated;
use winnow::PResult;
use winnow::Parser;

type Stream<'i> = &'i str;
// 3812ce95 ends here

// [[file:../../extxyz.note::739400bd][739400bd]]
// Parse one line until line ending
fn parse_xyz_line<'s>(frame_text: &mut Stream<'s>) -> PResult<RawAtom<'s>> {
    use winnow::ascii::alpha1;
    use winnow::ascii::digit1;
    use winnow::ascii::line_ending;
    use winnow::ascii::space0;
    use winnow::ascii::till_line_ending;
    use winnow::combinator::delimited;
    use winnow::combinator::terminated;

    // element symbol or number
    let sym_or_num = alt((alpha1, digit1));
    let ele = cut_err(delimited(space0, sym_or_num, space1))
        .context(label("element symbol or number"))
        .parse_next(frame_text)?;
    let float = recognize_sci_float.try_map(|s| s.parse::<f64>());
    let xyz: Vec<_> = cut_err(separated(3, float, space1)).context(label("xyz coords")).parse_next(frame_text)?;
    let (_, extra) = cut_err((space0, till_line_ending)).context(label("xyz extra")).parse_next(frame_text)?;

    let atom = RawAtom {
        element: ele,
        position: xyz.try_into().unwrap(),
        extra,
    };
    Ok(atom)
}

// num_of_atoms, comment_line, atoms_list
fn parse_xyz_frame<'s>(frame_text: &mut Stream<'s>) -> PResult<(usize, &'s str, Vec<RawAtom<'s>>)> {
    use winnow::ascii::alpha1;
    use winnow::ascii::digit1;
    use winnow::ascii::line_ending;
    use winnow::ascii::space0;
    use winnow::ascii::till_line_ending;
    use winnow::combinator::delimited;
    use winnow::combinator::terminated;

    let natoms = delimited(space0, digit1, space0).try_map(|s: &str| s.parse::<usize>());
    let natoms = cut_err(terminated(natoms, line_ending))
        .context(label("num of atoms"))
        .parse_next(frame_text)?;
    let comment = cut_err(terminated(till_line_ending, line_ending))
        .context(label("comment line"))
        .parse_next(frame_text)?;
    // NOTE: We supposed that there is no line ending in the end of `frame_text`
    let atoms: Vec<_> = separated(1.., cut_err(parse_xyz_line), line_ending)
        .context(label("atom list"))
        .parse_next(frame_text)?;

    Ok((natoms, comment, atoms))
}
// 739400bd ends here

// [[file:../../extxyz.note::690b8cfd][690b8cfd]]
impl<'s> RawAtom<'s> {
    /// Parse `RawAtom` from xyz line `input` in xyz format.
    pub fn parse_from(input: &'s str) -> anyhow::Result<Self> {
        use anyhow::anyhow;

        let atom = parse_xyz_line
            .parse(input.trim_end())
            .map_err(|e| anyhow!("parse xyz atom errors:\n{:}\ninput={input:?}", e.to_string()))?;
        Ok(atom)
    }
}

impl<'s> RawAtoms<'s> {
    /// Parse `RawAtoms` from a complete xyz frame `input` in xyz format.
    pub fn parse_from(input: &'s str) -> anyhow::Result<Self> {
        use anyhow::anyhow;

        let (natoms, comment, atoms) = parse_xyz_frame
            // Remove the trailing new lines, so that do not break `separated` parser
            .parse(input.trim_end())
            .map_err(|e| anyhow!("parse xyz atoms error:\n{}\ninput={input:?}", e.to_string()))?;
        Ok(Self { natoms, comment, atoms })
    }
}
// 690b8cfd ends here

// [[file:../../extxyz.note::1978c77e][1978c77e]]
#[test]
fn test_parse_extxyz_frame() -> anyhow::Result<()> {
    let input = "C        3.66613204       0.98814863       0.17310000";
    let atom = RawAtom::parse_from(input)?;
    assert_eq!(atom.element, "C");

    let input = "C        3.66613204       0.98814863D0     0.17310000 extra F\n";
    let atom = RawAtom::parse_from(input)?;
    assert_eq!(atom.extra, "extra F");

    let input = r#" 128
comment
C        3.66613204       0.98814863       0.17310000
C        1.89018534       2.42153712       3.34052440
C        2.66063880       4.93469322       3.30108500
 "#;
    let atoms = RawAtoms::parse_from(input)?;
    assert_eq!(atoms.natoms, 128);
    assert_eq!(atoms.atoms.len(), 3);

    Ok(())
}
// 1978c77e ends here
