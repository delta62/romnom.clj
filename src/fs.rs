use crate::error::{Error, Result};
use std::fs::ReadDir;
use std::path::Path;

pub fn all_files<P: AsRef<Path>>(path: P) -> Result<ReadDir> {
    std::fs::read_dir(path).map_err(|_| Error::IoError)
}

pub fn stat<P: AsRef<Path>>(path: P) -> Result<u64> {
    std::fs::metadata(path)
        .map(|m| m.len())
        .map_err(|_| Error::IoError)
}

pub fn copy(from: &Path, to: &Path) -> Result<()> {
    Err(Error::IoError)
}
