// [[file:../extxyz.note::7d01bbbd][7d01bbbd]]
use std::path::Path;

use anyhow::*;
use grep_reader::GrepReader;
// 7d01bbbd ends here

// [[file:../extxyz.note::bc363bfe][bc363bfe]]
/// Return an iterator over strings of selected frames in xyz/extxyz
/// format from trajectory in `path`. Large trajectory file is well
/// supported.
///
/// # Parameters
/// * path: path to the trajectory file
/// * selection: an iterator over indices of selected frames
pub fn read_xyz_frames(path: impl AsRef<Path>, mut selection: impl Iterator<Item = usize>) -> Result<impl Iterator<Item = String>> {
    let mut reader = GrepReader::try_from_path(path.as_ref())?;
    // allow whitespace before or after number
    let n = reader.mark(r"^\s*\d+\s*$", None)?;

    let frames = std::iter::from_fn(move || {
        if reader.current_marker() <= n {
            let j = selection.next()?;
            if j < n {
                reader.goto_marker(j).ok()?;
                let mut buf = String::new();
                reader.read_until_next_marker(&mut buf).ok()?;
                Some(buf)
            } else {
                None
            }
        } else {
            None
        }
    });

    Ok(frames)
}
// bc363bfe ends here
