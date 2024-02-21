// [[file:../extxyz.note::eb2a32cd][eb2a32cd]]
// #![deny(warnings)]
#![deny(clippy::all)]
// #![deny(missing_docs)]
// eb2a32cd ends here

//! Read large trajectory file in xyz/extxyz format
//!
//! # Example
//!
//! ```rust,ignore,no_run
//! use extxyz::{read_xyz_frames, RawAtoms, Info};
//! 
//! fn main() -> anyhow::Result<()> {
//!     // a large xyz/extxyz trajectory file
//!     let f = "nmd.xyz";
//!     // skip the first 100 frames, and read frames with a step size `10`
//!     let selection = (100..).step_by(10);
//!     let frames = read_xyz_frames(f, selection)?;
//!     for frame in frames {
//!         let atoms = RawAtoms::parse_from(&frame)?;
//!         // it will returen error if the comment is not in normal extxyz format
//!         let info: Info = atoms.comment.parse()?;
//!         // get molecule's properties
//!         let energy = info.get("energy").unwrap();
//!         // get atom's properties
//!         for atom in atoms.atoms {
//!             // parse extra data for each atom
//!             let atom_properties = info.parse_extra_columns(&atom.extra)?;
//!             // get `forces` component for each atom
//!             let forces = &atom_properties["forces"];
//!         }
//!     }
//! 
//!     Ok(())
//! }
//! ```

// [[file:../extxyz.note::10e3ae82][10e3ae82]]
mod parser;
mod trajectory;
// 10e3ae82 ends here

// [[file:../extxyz.note::bf78776e][bf78776e]]
#[cfg(feature = "adhoc")]
/// Docs for local mods
pub mod docs {
    macro_rules! export_doc {
        ($l:ident) => {
            pub mod $l {
                pub use crate::$l::*;
            }
        };
    }

    export_doc!(parser);
}
// bf78776e ends here

// [[file:../extxyz.note::bf0a7abd][bf0a7abd]]
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Represents the parsed atom in raw xyz format
pub struct RawAtom<'s> {
    /// Element symbol or number
    pub element: &'s str,
    /// The Cartesian coordinates
    pub position: [f64; 3],
    /// Any rest input other than above
    pub extra: &'s str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Represents the parsed atoms in raw xyz format
pub struct RawAtoms<'s> {
    /// The number of atoms
    pub natoms: usize,
    /// The content of the comment line
    pub comment: &'s str,
    /// The atom list parsed in the remaining lines
    pub atoms: Vec<RawAtom<'s>>,
}
// bf0a7abd ends here

// [[file:../extxyz.note::c3a71075][c3a71075]]
pub use crate::trajectory::*;

pub use crate::parser::extxyz::Info;
// c3a71075 ends here
