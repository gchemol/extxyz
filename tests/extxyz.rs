// [[file:../extxyz.note::c53397f5][c53397f5]]
use extxyz::{read_xyz_frames, Info, RawAtoms};

#[test]
fn test_extxyz() -> anyhow::Result<()> {
    let f = "tests/files/cu.xyz";
    let frames: Vec<_> = read_xyz_frames(f, 0..)?.collect();
    assert_eq!(frames.len(), 1);
    let frame = &frames[0];
    let atoms = RawAtoms::parse_from(frame)?;
    // it will returen error if the comment is not in normal extxyz format
    let info: Info = atoms.comment.parse()?;
    // dbg!(&info);
    // get molecule's properties
    let energy = info.get("energy").unwrap();
    assert_eq!(energy, 0.63);
    let user_data = info.get("user-data").unwrap();
    assert_eq!(user_data[0], 1);
    let pbc = info.get("pbc").unwrap();
    assert_eq!(pbc[0], true);
    // get atom's properties
    for atom in atoms.atoms {
        // parse extra data for each atom
        let atom_properties = info.parse_extra_columns(&atom.extra)?;
        // get `forces` from extra_data_dict
        let forces = &atom_properties["forces"];
        assert!(forces[0].is_f64());
        let energy = &atom_properties["energy"];
        assert!(energy.is_f64());
    }

    Ok(())
}
// c53397f5 ends here
