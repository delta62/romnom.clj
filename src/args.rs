use crate::types::Locale;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Locales to include
    #[arg(short, long)]
    pub locale: Vec<Locale>,

    /// Directory to search for ROMs. Directories are not searched recursively.
    #[arg(short, long)]
    pub path: String,

    /// Include ROMs marked as bad dumps
    #[arg(short, long)]
    pub bad_dumps: bool,

    /// The path to move ROMs to
    #[arg(short, long, required_if_eq("dry_run", "false"))]
    pub output: Option<String>,

    /// Only calculate what would be copied, but don't actually copy anything
    #[arg(short, long)]
    pub dry_run: bool,

    /// File extensions to copy. If omitted, all matching files are copied.
    #[arg(short, long)]
    pub extension: Vec<String>,

    /// Include ROMs marked as videos
    #[arg(short, long)]
    pub videos: bool,
}
