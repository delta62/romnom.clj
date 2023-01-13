use crate::error::{Error, Result};
use std::fs::Metadata;
use std::path::Path;
use tokio::fs;

pub async fn read_dir<P: AsRef<Path>>(path: P) -> Result<fs::ReadDir> {
    fs::read_dir(path).await.map_err(|_| Error::IoError)
}

pub async fn stat<P: AsRef<Path>>(path: P) -> Result<Metadata> {
    fs::metadata(path).await.map_err(|_| Error::IoError)
}

pub async fn copy(from: &Path, to: &Path) -> Result<()> {
    fs::copy(from, to)
        .await
        .map(|_| ())
        .map_err(|_| Error::IoError)
}
