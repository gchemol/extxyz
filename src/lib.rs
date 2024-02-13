// [[file:../extxyz.note::eb2a32cd][eb2a32cd]]
// #![deny(warnings)]
// #![deny(clippy::all)]
// eb2a32cd ends here

// [[file:../extxyz.note::*mods][mods:1]]
mod parser;
// mods:1 ends here

// [[file:../extxyz.note::1a12bd10][1a12bd10]]
#[test]
fn test_extxyz() {
    let s = r#"Lattice="5.44 0.0 0.0 0.0 5.44 0.0 0.0 0.0 5.44" Properties=species:S:1:pos:R:3 Time=0.0 good =a"#;
    let parts = shlex::split(s);
    dbg!(parts);

    let xyz = r#"2
Properties=species:S:1:pos:R:3:forces:R:3 energy=0.7943275145502244 pbc="F F F"
I        6.70077400       5.99240300      -1.02271700       0.03244218       0.02902981      -0.00495554
I       -6.70077400      -5.99240300       1.02271700      -0.03244270      -0.02902879       0.00495606
"#;
}
// 1a12bd10 ends here

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
