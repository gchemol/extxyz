// [[file:../extxyz.note::eb2a32cd][eb2a32cd]]
// #![deny(warnings)]
#![deny(clippy::all)]
// eb2a32cd ends here

// [[file:../extxyz.note::10e3ae82][10e3ae82]]
mod frame;
mod parser;
// 10e3ae82 ends here

// [[file:../extxyz.note::*mods][mods:1]]

// mods:1 ends here

// [[file:../extxyz.note::*docs][docs:1]]
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

    // export_doc!(codec);
}
// docs:1 ends here
