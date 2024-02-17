// [[file:../../extxyz.note::bf987e7c][bf987e7c]]
use crate::parser::recognize_sci_float;
use crate::{RawAtom, RawAtoms};

use winnow::ascii::{space0, space1};
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::separated;
use winnow::PResult;
use winnow::Parser;

type Stream<'i> = &'i str;
// bf987e7c ends here
