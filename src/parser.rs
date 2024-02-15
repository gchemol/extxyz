// [[file:../extxyz.note::51ad662c][51ad662c]]
use winnow::ascii::till_line_ending;
use winnow::ascii::{alphanumeric1, space0, space1};
use winnow::combinator::alt;
use winnow::combinator::{delimited, separated};
use winnow::combinator::{not, opt, repeat, todo};
use winnow::prelude::*;
use winnow::stream::AsChar;
use winnow::token::take_till;
use winnow::token::take_while;

pub type Stream<'i> = &'i str;
// 51ad662c ends here

// [[file:../extxyz.note::1a36024d][1a36024d]]
fn recognize_boolean<'i>(input: &mut Stream<'i>) -> PResult<&'i str> {
    let parse_true = alt(("true", "TRUE", "T", "True")).value("true");
    let parse_false = alt(("false", "FALSE", "F", "False")).value("false");
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
fn recognize_float<'i>(input: &mut Stream<'i>) -> PResult<String> {
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
    let (_, v) = recognize_float.try_map(|s| s.parse::<f64>()).parse_peek(s)?;
    assert_eq!(v, -1.234);

    let s = "-12";
    let (_, v) = recognize_float.try_map(|s| s.parse::<f64>()).parse_peek(s)?;
    assert_eq!(v, -12.0);

    let s = "-12.3E-1";
    let (_, v) = recognize_float.try_map(|s| s.parse::<f64>()).parse_peek(s)?;
    assert_eq!(v, -1.23);

    Ok(())
}
// 1a36024d ends here

// [[file:../extxyz.note::ce5ca27d][ce5ca27d]]
// one key=value pair on second comment line
fn key_value<'s>(i: &mut Stream<'s>) -> PResult<(Stream<'s>, Stream<'s>)> {
    // Key: bare or quoted string
    let key = alt((quoted_string, alphanumeric1)).parse_next(i)?;
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
fn key_value_pairs<'s>(input: &mut Stream<'s>) -> PResult<Vec<(Stream<'s>, Stream<'s>)>> {
    let r = separated(0.., key_value, space1).parse_next(input)?;
    Ok(r)
}

#[test]
fn test_key_value() -> PResult<()> {
    let s = r#"Lattice="5.44 0.0 0.0 0.0 5.44 0.0 0.0 0.0 5.44" Properties=species:S:1:pos:R:3 Time=0.0"#;
    let (_, r) = key_value_pairs.parse_peek(s)?;
    assert_eq!(r.len(), 3);
    Ok(())
}
// ce5ca27d ends here

// [[file:../extxyz.note::dd9bed2f][dd9bed2f]]
fn recognize_old_style_array<'i>(input: &mut Stream<'i>) -> PResult<Vec<String>> {
    use winnow::ascii::space1;
    use winnow::token::one_of;

    // separated by commas and optional whitespace, or
    // separated by whitespace
    let comma = (",", opt(space1)).value(",");
    let sep = alt((space1, comma));
    let mut list_values = separated(1.., recognize_float, sep);
    let values = delimited(opt(one_of(['[', '{'])), list_values, opt(one_of([']', '}']))).parse_next(input)?;
    Ok(values)
}

#[test]
fn test_parse_1d_array() -> PResult<()> {
    let input = "5.44 0.0 0.0 0.0 5.44 0.0 0.0 0.0 5.44";
    let (_, x) = recognize_old_style_array.parse_peek(input)?;
    assert_eq!(x.len(), 9);

    let input = "[5.44 0.0 0.0 0.0 5.44 0.0 0.0 0.0 5.44]";
    let (_, x) = recognize_old_style_array.parse_peek(input)?;

    let input = "{5.44 0.0 0.0 0.0 5.44 0.0 0.0 0.0 5.44}";
    let (_, x) = recognize_old_style_array.parse_peek(input)?;

    assert_eq!(x.len(), 9);
    let input = "[5.44, 0.0,0.0,0.0,5.44,0.0,0.0,0.0,5.44]";
    let (_, x) = recognize_old_style_array.parse_peek(input)?;
    assert_eq!(x.len(), 9);
    Ok(())
}
// dd9bed2f ends here

// [[file:../extxyz.note::9a7ccb4b][9a7ccb4b]]
#[derive(Debug, PartialEq)]
struct PropertyValue {
    name: String,
    r#type: ValueType,
    num_columns: usize,
}

#[derive(Debug, PartialEq)]
enum ValueType {
    String,
    Integer,
    Logical,
    Real,
}

impl ValueType {
    fn new(t: char) -> Self {
        match t {
            'S' => Self::String,
            'I' => Self::Integer,
            'R' => Self::Real,
            'L' => Self::Logical,
            _ => todo!(),
        }
    }
}

fn property_value<'i>(input: &mut Stream<'i>) -> PResult<PropertyValue> {
    use winnow::ascii::alphanumeric1;
    use winnow::ascii::digit1;
    use winnow::combinator::seq;
    use winnow::combinator::terminated;
    use winnow::token::one_of;

    // names the column(s)
    let name = alphanumeric1;
    // indicates the type in the column
    let t_columns = one_of(['S', 'I', 'R', 'L']);
    // specifying how many consecutive columns are being referred to
    let n_columns = digit1;
    let (name, type_char, num) = (terminated(name, ":"), terminated(t_columns, ":"), n_columns).parse_next(input)?;
    Ok(PropertyValue {
        name: name.to_string(),
        r#type: ValueType::new(type_char),
        num_columns: num.parse().unwrap(),
    })
}

fn parse_property_values<'i>(input: &mut Stream<'i>) -> PResult<Vec<PropertyValue>> {
    separated(1.., property_value, ":").parse_next(input)
}

#[test]
fn test_parse_properties() -> PResult<()> {
    let input = "species:S:1:pos:R:3";
    let (_, properties) = parse_property_values.parse_peek(input)?;
    assert_eq!(properties.len(), 2);
    Ok(())
}
// 9a7ccb4b ends here

// [[file:../extxyz.note::68a854b3][68a854b3]]
use serde::Deserialize;
use serde_json::Value;

struct Info {
    dict: serde_json::Map<String, Value>,
}

#[test]
fn test_info() {
    let x = "0.0";
    let v: Value = x.parse().unwrap();
    dbg!(v);

    let x = "[1, 2, 3]";
    let v: Value = x.parse().unwrap();
    dbg!(v);

    let x = "[[5.44, 0.0], [1.0], [2, 3]]";
    let v: Value = x.parse().unwrap();
    dbg!(v);

    let x = "true";
    let v: Value = x.parse().unwrap();
    dbg!(v);

    // let mut x = Info::new();
    // x.insert("12".into(), (1).into());
    // x.insert("Time".into(), (0.0).into());
    // dbg!(x);
}
// 68a854b3 ends here
