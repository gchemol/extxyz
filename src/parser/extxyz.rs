// [[file:../../extxyz.note::bf987e7c][bf987e7c]]
use anyhow::anyhow;
use serde_json::json;

use super::{label, recognize_boolean, recognize_integer, recognize_sci_float, Stream};
use crate::{RawAtom, RawAtoms};

use winnow::ascii::{space0, space1};
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::separated;
use winnow::PResult;
use winnow::Parser;
// bf987e7c ends here

// [[file:../../extxyz.note::823b4ece][823b4ece]]
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

/// Represents the data parsed from extxyz comment line.
///
/// Example input: Lattice="5.44 0.0 0.0 0.0 5.44 0.0 0.0 0.0 5.44" Properties=species:S:1:pos:R:3 Time=0.0
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Info {
    /// A heterogeneous map (dict) for parsed key-value pairs
    dict: serde_json::Map<String, Value>,
}

/// Represents one value item for special key "Properties", which
/// defines the columns in the subsequent lines in the frame.
///
/// Example input: Properties=species:S:1:pos:R:3
#[derive(Debug, Serialize)]
pub struct PropertyValue {
    pub name: String,
    pub r#type: PropertyValueType,
    pub num_columns: usize,
}

/// Represents the column value type in `Properties` values
#[derive(Debug, Serialize)]
pub enum PropertyValueType {
    /// S
    String,
    /// I
    Integer,
    /// L
    Logical,
    /// R
    Real,
}

impl PropertyValueType {
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
// 823b4ece ends here

// [[file:../../extxyz.note::9a7ccb4b][9a7ccb4b]]
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
        r#type: PropertyValueType::new(type_char),
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

// [[file:../../extxyz.note::9ecc3cf5][9ecc3cf5]]
// extract "Lattice" entry and apply semantic conversions
// 9ecc3cf5 ends here

// [[file:../../extxyz.note::78659ab1][78659ab1]]
fn parse_extra_atom_data(input: &str, info: &Info) -> anyhow::Result<serde_json::Map<String, Value>> {
    use winnow::combinator::preceded;
    use winnow::error::ContextError;
    use winnow::error::ErrMode;

    use PropertyValueType::*;

    let mut map = serde_json::Map::new();
    let mut s = input.trim();
    let atom_properties = info.get_properties()?;
    let e_any = |e: ErrMode<ContextError>| anyhow!(e.to_string());
    for col in atom_properties {
        let mut real_value = preceded(space0, recognize_sci_float).try_map(|x| x.parse::<f64>());
        let mut bool_value = preceded(space0, recognize_boolean).try_map(|x| x.parse::<bool>());
        let mut isize_value = preceded(space0, recognize_integer).try_map(|x| x.parse::<isize>());
        let value = match (&col.name[..], col.r#type, col.num_columns) {
            // ignore element and positions columns
            ("species" | "pos", _, _) => Value::Null,
            (_, Real, 1) => json!(real_value.parse_next(&mut s).map_err(e_any)?),
            (_, Integer, 1) => json!(isize_value.parse_next(&mut s).map_err(e_any)?),
            (_, Logical, 1) => json!(bool_value.parse_next(&mut s).map_err(e_any)?),
            (_, String, 1) => json!(bare_string.parse_next(&mut s).map_err(e_any)?),
            (_, Real, num_col) if num_col > 1 => {
                let v: Vec<_> = separated(num_col, real_value, space1).parse_next(&mut s).map_err(e_any)?;
                json!(v)
            }
            (_, Integer, num_col) if num_col > 1 => {
                let v: Vec<_> = separated(num_col, isize_value, space1).parse_next(&mut s).map_err(e_any)?;
                json!(v)
            }
            (_, Logical, num_col) if num_col > 1 => {
                let v: Vec<_> = separated(num_col, bool_value, space1).parse_next(&mut s).map_err(e_any)?;
                json!(v)
            }
            (_, String, num_col) if num_col > 1 => {
                let v: Vec<_> = separated(num_col, bare_string, space1).parse_next(&mut s).map_err(e_any)?;
                json!(v)
            }
            _ => anyhow::bail!("invalid column data ({s:?}) or column info: {info:?}"),
        };
        if value != Value::Null {
            map.insert(col.name.into(), value);
        }
    }

    Ok(map)
}
// 78659ab1 ends here

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

// [[file:../../extxyz.note::dd9bed2f][dd9bed2f]]
fn recognize_old_one_d_array<'i>(input: &mut Stream<'i>) -> PResult<Vec<String>> {
    use winnow::ascii::space1;
    use winnow::token::one_of;

    // separated by commas and optional whitespace, or
    // separated by whitespace
    let comma = (",", opt(space1)).value(",");
    let sep = alt((space1, comma));
    let boolean = recognize_boolean.map(|s| s.to_string());
    let string = parse_string.map(|s| s.to_string());
    let mut list_values = separated(2.., alt((recognize_sci_float, boolean, string)), sep);
    let values = delimited(opt(one_of(['[', '{'])), list_values, opt(one_of([']', '}']))).parse_next(input)?;
    Ok(values)
}

#[test]
fn test_parse_1d_array() -> PResult<()> {
    let input = "5.44 0.0 0.0 0.0 5.44 0.0 0.0 0.0 5.44";
    let (_, x) = recognize_old_one_d_array.parse_peek(input)?;
    assert_eq!(x.len(), 9);

    let input = "[5.44 0.0 0.0 0.0 5.44 0.0 0.0 0.0 5.44]";
    let (_, x) = recognize_old_one_d_array.parse_peek(input)?;

    let input = "{5.44 0.0 0.0 0.0 5.44 0.0 0.0 0.0 5.44}";
    let (_, x) = recognize_old_one_d_array.parse_peek(input)?;
    assert_eq!(x.len(), 9);

    let input = "[5.44, 0.0,0.0,0.0,5.44,0.0,0.0,0.0,5.44]";
    let (_, x) = recognize_old_one_d_array.parse_peek(input)?;
    assert_eq!(x.len(), 9);

    // bool type vector
    let input = "T T T";
    let (_, x) = recognize_old_one_d_array.parse_peek(input)?;
    assert_eq!(x.len(), 3);

    // not a vector
    let input = "T";
    let r = recognize_old_one_d_array.parse_peek(input);
    assert!(r.is_err());

    Ok(())
}
// dd9bed2f ends here

// [[file:../../extxyz.note::1e59b3a0][1e59b3a0]]
fn reformat_old_style_array(input: Stream) -> PResult<String> {
    let (rest, ss) = recognize_old_one_d_array.parse_peek(input)?;
    // Speical case: not-list="1.2 2 some"
    if rest.is_empty() {
        let reformatted: String = format!("[{}]", ss.join(", "));
        Ok(reformatted)
    } else {
        Ok(input.to_string())
    }
}

fn reformat_extxyz_value(input: &str) -> String {
    if let Ok((rest, recognized)) = recognize_sci_float.parse_peek(input) {
        if rest.is_empty() {
            return recognized;
        }
    }

    if let Ok((rest, recognized)) = recognize_boolean.parse_peek(input) {
        if rest.is_empty() {
            return recognized.to_string();
        }
    }

    if let Ok(recognized) = reformat_old_style_array(input) {
        return recognized.into();
    }

    input.to_string()
}

#[test]
fn test_extxyz_value() -> PResult<()> {
    let s = "True";
    let x = reformat_extxyz_value(s);
    assert_eq!(x, "true");

    let s = "-12.4D-5";
    let x = reformat_extxyz_value(s);
    assert_eq!(x, "-12.4E-5");

    let s = "-12";
    let x = reformat_extxyz_value(s);
    assert_eq!(x, "-12");

    let s = "T T T";
    let x = reformat_extxyz_value(s);
    assert_eq!(x, "[true, true, true]");

    Ok(())
}
// 1e59b3a0 ends here

// [[file:../../extxyz.note::68a854b3][68a854b3]]
fn parse_extxyz_title<'s>(title: &mut Stream<'s>) -> PResult<Info> {
    let kv_pairs = parse_key_value_pairs.parse_next(title)?;

    let mut info = Info::default();
    for (k, v) in kv_pairs {
        let mut v = reformat_extxyz_value(v);
        let k = k.to_string();
        // ASE style: user-data="_JSON [1, 2, 3]"
        if v.starts_with("_JSON ") {
            v = v[5..].to_string();
        }
        // convert value in string to json typed `Value`
        if let Ok(v_parsed) = v.parse::<Value>() {
            info.dict.insert(k, v_parsed);
        } else {
            info.dict.insert(k, v.into());
        }
    }

    Ok(info)
}

#[test]
fn test_parse_extxyz_title() -> PResult<()> {
    let mut s = r#"Lattice="5.44 0.0 0.0 0.0 5.44 0.0 0.0 0.0 5.44" Properties=species:S:1:pos:R:3 Time=0.0 pbc="T T T""#;
    let info = parse_extxyz_title(&mut s)?;
    assert_eq!(info.dict["Time"], 0.0);
    assert_eq!(info.dict["pbc"][0], true);

    let mut s = r#"s=string b1=True b2= F real=3.14 integer=-314 array="[1,2,3]""#;
    let info = parse_extxyz_title(&mut s)?;
    assert_eq!(info.dict["real"], 3.14);
    assert_eq!(info.dict["s"], "string");
    assert_eq!(info.dict["integer"], -314);
    assert_eq!(info.dict["b1"], true);
    assert_eq!(info.dict["b2"], false);
    assert_eq!(info.dict["array"][0], 1);

    let mut s = r#""real quoted"="3.14" array="1.2 2 3" nested="[[1], [2], [3]]" special="1 2 T""#;
    let info = parse_extxyz_title(&mut s)?;
    assert_eq!(info.dict["real quoted"], 3.14);
    assert_eq!(info.dict["array"][0], 1.2);
    assert_eq!(info.dict["nested"][0][0], 1);
    assert_eq!(info.dict["special"][0], 1);
    assert_eq!(info.dict["special"][2], true);

    let mut s = r#"Lattice="10.83 0.0 0.0 0.0 10.83 0.0 0.0 0.0 10.83" Properties=forces:R:3:energies:R:1 user-data="_JSON [1, 2, 3]" energy=0.634"#;
    let info = parse_extxyz_title(&mut s)?;
    assert!(info.dict["user-data"][0].is_u64());

    Ok(())
}
// 68a854b3 ends here

// [[file:../../extxyz.note::a15396a3][a15396a3]]
impl std::str::FromStr for Info {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_extxyz_title.parse(input).map_err(|e| {
            let error = e.to_string();
            anyhow::anyhow!("Parse extxyz comment line failure:\n{error}\ninput={input:?}")
        })
    }
}

impl Info {
    /// Returns the associated value of `key`.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.dict.get(key)
    }

    /// Removes and returns an value from `Info` having the given key.
    pub fn pop(&mut self, key: &str) -> Option<Value> {
        self.dict.remove(key)
    }

    /// Return parsed per-atom properties
    fn get_properties(&self) -> anyhow::Result<Vec<PropertyValue>> {
        let properties = if let Some(Value::String(properties)) = self.dict.get("Properties") {
            properties
        } else {
            "species:S:1:pos:R:3"
        };
        let property_values = parse_property_values.parse(properties).map_err(|e| {
            let error = e.to_string();
            anyhow::anyhow!("failed to parse extxyz properties:\n{error}\ninput={properties:?}")
        })?;

        Ok(property_values)
    }

    /// Return reference to inner `Map`
    pub fn raw_map(&self) -> &serde_json::Map<String, Value> {
        &self.dict
    }

    /// Mut access to inner `Map`
    pub fn raw_map_mut(&mut self) -> &mut serde_json::Map<String, Value> {
        &mut self.dict
    }
}

impl Info {
    /// Parse atom properties from extra columns in `extra`.
    pub fn parse_extra_columns(&self, extra: &str) -> anyhow::Result<serde_json::Map<String, Value>> {
        parse_extra_atom_data(extra, &self)
    }
}
// a15396a3 ends here

// [[file:../../extxyz.note::b4d166a0][b4d166a0]]
#[test]
fn test_extxyz_info() -> anyhow::Result<()> {
    let s = r#"Lattice="5.44 0.0 0.0 0.0 5.44 0.0 0.0 0.0 5.44" Properties=species:S:1:pos:R:3:forces:R:3:freeze:L:1 Time=0.0"#;
    let info: Info = s.parse()?;
    assert_eq!(info.get("Time").unwrap(), 0.0);
    assert_eq!(info.get("Lattice").unwrap()[0], 5.44);

    let extra = "       0.03244218       0.02902981      -0.00495554 F";
    let atom_properties = info.parse_extra_columns(extra)?;
    assert_eq!(atom_properties["forces"][0], 0.03244218);
    assert_eq!(atom_properties["freeze"], false);

    Ok(())
}
// b4d166a0 ends here
