A fast parser for chemical file in the extended xyz ([extxyz](https://github.com/libAtoms/extxyz?tab=readme-ov-file#general-definitions)) format.

The [extended XYZ format](https://www.ovito.org/docs/current/reference/file_formats/input/xyz.html#extended-xyz-format) is an enhanced version of the simple XYZ
format, allowing extra columns to be present in the file for extra
atom properties (forces, charges, labels, etc.) as well as molecule's
properties (energy, dipole moment, etc.)specified in the comment line.


# Usage

Example usage:

    use extxyz::{read_xyz_frames, RawAtoms};
    
    fn main() -> anyhow::Result<()> {
        use super::RawAtoms;
    
        let f = "nmd.xyz";
        // skip the first 100 frames, and read frames with a step size `10`
        let selection = (100..).step_by(10);
        let frames = read_xyz_frames(f, selection)?;
        for frame in frames {
            let atoms = RawAtoms::parse_from(&frame)?;
            // it will returen error if the comment is not in normal extxyz format
            let info: Info = atoms.comment.parse()?;
            // get molecule's properties
            let energy = info.get("energy").unwrap();
            // get atom's properties
            for atom in atoms {
                // parse extra data for each atom
                let extra_data = info.parse_extra_columns(&atom.extra)?;
                // get `forces` from extra_data_dict
                let forces = extra_data.get("forces").unwrap();
            }
        }
    
        Ok(())
    }

