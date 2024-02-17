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
//! use extxyz::*;
//! 
//! let xyzfile = "large-trajectory.xyz";
//! let frames = read_xyz_frames(xyzfile, (2270..).step_by(2))?;
//! for frame in frames {
//!     let atoms = RawAtoms::parse_from(&frame)?;
//!     dbg!(atoms);
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

    export_doc!(trajectory);
}
// bf78776e ends here

// [[file:../extxyz.note::bf0a7abd][bf0a7abd]]
#[derive(Debug)]
/// Represents the parsed atom in raw xyz format
pub struct RawAtom<'s> {
    /// Element symbol or number
    pub element: &'s str,
    /// The Cartesian coordinates
    pub positions: [f64; 3],
    /// Any rest input other than above
    pub extra: &'s str,
}

#[derive(Debug)]
/// Represents the parsed atoms in raw xyz format
pub struct RawAtoms<'s> {
    /// The number of atoms
    pub natoms: usize,
    /// The comment in the second line
    pub comment: &'s str,
    /// The atom list parsed in the remaining lines
    pub atoms: Vec<RawAtom<'s>>,
}
// bf0a7abd ends here

// [[file:../extxyz.note::c3a71075][c3a71075]]
pub use crate::trajectory::*;
// c3a71075 ends here
