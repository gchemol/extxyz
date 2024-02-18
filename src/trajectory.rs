// [[file:../extxyz.note::7d01bbbd][7d01bbbd]]
// #![deny(warnings)]
#![deny(clippy::all)]
#![deny(missing_docs)]

use std::path::Path;

use anyhow::*;
use grep_reader::GrepReader;
// 7d01bbbd ends here

// [[file:../extxyz.note::55fa400b][55fa400b]]
mod reader {
    // #![deny(warnings)]
    // #![deny(clippy::all)]
    // #![deny(missing_docs)]
    
    use anyhow::*;

    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::Read;
    use std::path::Path;
    use std::result::Result::Ok;

    type FileReader = BufReader<File>;

    fn text_file_reader<P: AsRef<Path>>(p: P) -> Result<FileReader> {
        let p = p.as_ref();
        let f = File::open(p).with_context(|| format!("Failed to open file {:?}", p))?;

        let reader = BufReader::new(f);
        Ok(reader)
    }

    #[derive(Debug)]
    /// A stream reader for large text file
    pub struct TextReader<R> {
        inner: R,
    }

    impl TextReader<FileReader> {
        /// Build a text reader for file from path `p`.
        pub fn try_from_path(p: &Path) -> Result<Self> {
            let reader = text_file_reader(p)?;
            let parser = Self { inner: reader };
            Ok(parser)
        }
    }

    impl<R: Read> TextReader<BufReader<R>> {
        /// Build a text reader from a struct implementing Read trait.
        pub fn new(r: R) -> Self {
            Self { inner: BufReader::new(r) }
        }
    }

    impl<R: BufRead> TextReader<R> {
        /// Read a new line into buf.
        ///
        /// # NOTE
        /// - This function will return the total number of bytes read.
        /// - If this function returns None, the stream has reached EOF or encountered any error.
        pub fn read_line(&mut self, buf: &mut String) -> Option<usize> {
            match self.inner.read_line(buf) {
                Ok(0) => None,
                Ok(n) => Some(n),
                Err(_) => {
                    // discard any read in buf
                    return None;
                }
            }
        }

        /// Returns an iterator over the lines of this reader. Each string returned
        /// will not have a line ending.
        pub fn lines(self) -> impl Iterator<Item = String> {
            // silently ignore UTF-8 error
            self.inner.lines().filter_map(|s| if let Ok(line) = s { Some(line) } else { None })
        }

        /// Read all text into string `buf` (Note: out of memory issue for large
        /// file)
        pub fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
            let n = self.inner.read_to_string(buf)?;
            Ok(n)
        }
    }
}
// 55fa400b ends here

// [[file:../extxyz.note::48f5accb][48f5accb]]
/// Return an iterator that yields strings of the selected frames in the
/// `xyz/extxyz` format from trajectory in `path`. Reading frames
/// selectively is suitable for large trajectory files.
///
/// # NOTE
/// * The first line (an integer) in `xyz` frame is used as a frame
///   separator. The following atom lines can be more or fewer than
///   specified. That is, `VEC` atom line specification is well supported.
///
/// # Parameters
/// * `path`: path to the trajectory file
/// * `selection`: an iterator over indices of selected frames
pub fn read_xyz_frames_two_pass(path: impl AsRef<Path>, mut selection: impl Iterator<Item = usize>) -> Result<impl Iterator<Item = String>> {
    let mut reader = GrepReader::try_from_path(path.as_ref())?;

    // pass1: mark natoms lines using grep
    // allow whitespace before or after number
    let n = reader.mark(r"^\s*\d+\s*$", None)?;

    // pass2: read frames selectively
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
// 48f5accb ends here

// [[file:../extxyz.note::bc363bfe][bc363bfe]]
pub use read_xyz_frames_two_pass as read_xyz_frames;
// bc363bfe ends here

// [[file:../extxyz.note::d3eeabd9][d3eeabd9]]
/// Return an iterator that yields strings of the selected frames in the
/// `xyz/extxyz` format from trajectory in `path`. Supports large
/// trajectory files.
///
/// # NOTE
/// * The first line in `xyz` frame should be the real number of atom
///   lines. That is, `VEC` atom line specifications are not supported.
pub fn read_xyz_frames_direct(path: impl AsRef<Path>) -> Result<impl Iterator<Item = String>> {
    let mut reader = self::reader::TextReader::try_from_path(path.as_ref())?;
    let frames = std::iter::from_fn(move || {
        let mut buf = String::new();
        let _ = reader.read_line(&mut buf)?;
        let n: usize = buf.trim().parse().ok()?;
        let _ = reader.read_line(&mut buf)?;
        for _ in 0..n {
            reader.read_line(&mut buf)?;
        }
        Some(buf)
    });

    Ok(frames)
}
// d3eeabd9 ends here
