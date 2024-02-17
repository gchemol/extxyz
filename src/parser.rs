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

type Stream<'i> = &'i str;
// 51ad662c ends here

// [[file:../extxyz.note::a270017d][a270017d]]
mod extxyz;
mod xyz;
// a270017d ends here

// [[file:../extxyz.note::1a36024d][1a36024d]]
fn recognize_boolean<'i>(input: &mut Stream<'i>) -> PResult<&'i str> {
    let parse_true = alt(("true", "TRUE", "True", "T")).value("true");
    let parse_false = alt(("false", "FALSE", "False", "F")).value("false");
    alt((parse_true, parse_false)).parse_next(input)
}

/// string of one or more decimal digits, optionally preceded by sign
fn recognize_integer<'i>(input: &mut Stream<'i>) -> PResult<&'i str> {
    use winnow::ascii::digit1;
    use winnow::combinator::cut_err;
    use winnow::token::one_of;

    let r = (opt(one_of(['+', '-'])), cut_err(digit1)).recognize().parse_next(input)?;
    Ok(r)
}

/// string of one or more decimal digits, optionally preceded by sign
fn recognize_sci_float<'i>(input: &mut Stream<'i>) -> PResult<String> {
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

// [[file:../extxyz.note::ce5ca27d][ce5ca27d]]
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

/// key=value pairs on second ("comment") line
fn parse_key_value_pairs<'s>(input: &mut Stream<'s>) -> PResult<Vec<(Stream<'s>, Stream<'s>)>> {
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
