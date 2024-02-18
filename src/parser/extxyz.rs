// [[file:../../extxyz.note::bf987e7c][bf987e7c]]
use super::{label, recognize_sci_float, Stream};
use crate::{RawAtom, RawAtoms};

use winnow::ascii::{space0, space1};
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::separated;
use winnow::PResult;
use winnow::Parser;
// bf987e7c ends here

// [[file:../../extxyz.note::ce5ca27d][ce5ca27d]]
use winnow::combinator::delimited;
use winnow::combinator::opt;
use winnow::stream::AsChar;
use winnow::token::take_till;
use winnow::token::take_while;

fn bare_string<'s>(input: &mut Stream<'s>) -> PResult<Stream<'s>> {
    take_while(1.., ('a'..='z', 'A'..='z', '0'..='9', '_', '-')).parse_next(input)
}

// one key=value pair on second comment line
fn key_value<'s>(i: &mut Stream<'s>) -> PResult<(Stream<'s>, Stream<'s>)> {
    // Key: bare or quoted string
    let key = alt((quoted_string, bare_string)).parse_next(i)?;
    // spaces are allowed around = sign, which do not become part of the key or value.
    let _ = (opt(space0), "=", opt(space0)).parse_next(i)?;
    let mut normal_value = take_while(0.., not_whitespace);
    let val = alt((quoted_string, normal_value)).parse_next(i)?;
    Ok((key, val))
}

/// Parse a non-empty block of text that doesn't include \ or "
fn parse_string<'s>(input: &mut Stream<'s>) -> PResult<Stream<'s>> {
    let not_quote_slash = take_till(1.., ['"', '\\']);
    // ensure that the output of take_till is non-empty.
    let s = not_quote_slash.verify(|s: &str| !s.is_empty()).parse_next(input)?;
    Ok(s)
}

fn not_whitespace(chr: char) -> bool {
    !(chr.is_space() || chr.is_newline())
}

/// quoted string (starting and ending with double quote and containing only allowed characters)
fn quoted_string<'s>(input: &mut Stream<'s>) -> PResult<Stream<'s>> {
    let r = delimited('"', parse_string, '"').parse_next(input)?;
    Ok(r)
}

/// Parse key=value pairs in extxyz comment line
pub fn parse_key_value_pairs<'s>(input: &mut Stream<'s>) -> PResult<Vec<(Stream<'s>, Stream<'s>)>> {
    let r = separated(0.., key_value, space1).parse_next(input)?;
    Ok(r)
}

#[test]
fn test_key_value() -> PResult<()> {
    let s = r#"Lattice="5.44 0.0 0.0 0.0 5.44 0.0 0.0 0.0 5.44" Properties=species:S:1:pos:R:3 Time=0.0"#;
    let (_, r) = parse_key_value_pairs.parse_peek(s)?;
    assert_eq!(r.len(), 3);

    let s = r#"array="1 2 3" nested="[[1], [2], [3]]""#;
    let x = key_value.parse_peek(s);
    assert!(x.is_ok());

    let s = r#""real quoted"="3.14" array_complex="1 2 3" nested="[[1], [2], [3]]""#;
    let (_, r) = parse_key_value_pairs.parse_peek(s)?;
    assert_eq!(r.len(), 3);
    Ok(())
}
// ce5ca27d ends here
