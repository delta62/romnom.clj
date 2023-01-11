use crate::error::{Error, Result};
use std::fs::ReadDir;
use std::path::Path;

pub fn all_files<P: AsRef<Path>>(path: P) -> Result<ReadDir> {
    std::fs::read_dir(path).map_err(|_| Error::IoError)
}
