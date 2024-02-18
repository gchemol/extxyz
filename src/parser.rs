// [[file:../extxyz.note::51ad662c][51ad662c]]
// #![deny(warnings)]

use super::{RawAtom, RawAtoms};

use winnow::ascii::{alphanumeric1, space0, space1};
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::{delimited, separated};
use winnow::combinator::{not, opt, repeat, todo};
use winnow::prelude::*;
use winnow::stream::AsChar;
use winnow::token::take_till;
use winnow::token::take_while;
// 51ad662c ends here

// [[file:../extxyz.note::a270017d][a270017d]]
pub mod extxyz;
pub mod xyz;
// a270017d ends here

// [[file:../extxyz.note::91f20e0b][91f20e0b]]
use winnow::error::StrContext;

type Stream<'i> = &'i str;

fn label(s: &'static str) -> StrContext {
    StrContext::Label(s)
}
// 91f20e0b ends here

// [[file:../extxyz.note::1a36024d][1a36024d]]
/// Recognize one or more decimal digits
pub fn recognize_unsigned_integer<'i>(input: &mut Stream<'i>) -> PResult<&'i str> {
    use winnow::ascii::digit1;
    use winnow::combinator::cut_err;
    use winnow::token::one_of;

    let r = cut_err(digit1).recognize().parse_next(input)?;
    Ok(r)
}

/// Recognize normal string boolean type as `true` or `false`
pub fn recognize_boolean<'i>(input: &mut Stream<'i>) -> PResult<&'i str> {
    let parse_true = alt(("true", "TRUE", "True", "T")).value("true");
    let parse_false = alt(("false", "FALSE", "False", "F")).value("false");
    alt((parse_true, parse_false)).parse_next(input)
}

/// Recognize one or more decimal digits, optionally preceded by sign
pub fn recognize_integer<'i>(input: &mut Stream<'i>) -> PResult<&'i str> {
    use winnow::ascii::digit1;
    use winnow::combinator::cut_err;
    use winnow::token::one_of;

    let r = (opt(one_of(['+', '-'])), cut_err(digit1)).recognize().parse_next(input)?;
    Ok(r)
}

/// Recoginize normal float number. The D format code for scientific
/// (exponential) notation is also supported.
pub fn recognize_sci_float<'i>(input: &mut Stream<'i>) -> PResult<String> {
    use winnow::ascii::digit1;
    use winnow::combinator::cut_err;
    use winnow::combinator::preceded;
    use winnow::stream::Located;
    use winnow::token::one_of;

    // e.g. -1.34D+8
    let pre_exponent = (
        opt(one_of(['+', '-'])),
        alt(((digit1, opt(('.', opt(digit1)))).map(|_| ()), ('.', digit1).map(|_| ()))),
    )
        .recognize()
        .parse_next(input)?;

    let float_s = if let Some(exponent) = opt(preceded(one_of(['e', 'E', 'D', 'd']), recognize_integer)).parse_next(input)? {
        format!("{pre_exponent}E{exponent}")
    } else {
        format!("{pre_exponent}")
    };
    Ok(float_s)
}

#[test]
fn test_parse_value() -> PResult<()> {
    let s = "+12";
    let (_, v) = recognize_integer.try_map(|s| s.parse::<i32>()).parse_peek(s)?;
    assert_eq!(v, 12);

    let s = "-12";
    let (_, v) = recognize_integer.try_map(|s| s.parse::<i32>()).parse_peek(s)?;
    assert_eq!(v, -12);

    let s = "-12.34d-1";
    let (_, v) = recognize_sci_float.try_map(|s| s.parse::<f64>()).parse_peek(s)?;
    assert_eq!(v, -1.234);

    let s = "-12";
    let (_, v) = recognize_sci_float.try_map(|s| s.parse::<f64>()).parse_peek(s)?;
    assert_eq!(v, -12.0);

    let s = "-12.3E-1";
    let (_, v) = recognize_sci_float.try_map(|s| s.parse::<f64>()).parse_peek(s)?;
    assert_eq!(v, -1.23);

    Ok(())
}
// 1a36024d ends here

// [[file:../extxyz.note::9bbb5034][9bbb5034]]
// pub use self::extxyz::*;
// pub use self::xyz::*;
// 9bbb5034 ends here
