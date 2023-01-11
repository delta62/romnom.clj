use clap::Parser;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::{
    borrow::Cow,
    collections::HashSet,
    fs::{DirEntry, ReadDir},
    path::Path,
    str::FromStr,
};

#[derive(Debug)]
enum Error {
    IoError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Hash, Debug, clap::ValueEnum, PartialEq, Eq)]
enum Locale {
    UnitedStates,
    Europe,
    Japan,
}

impl Locale {
    fn represented_by(&self, s: &str) -> bool {
        match self {
            Self::UnitedStates => s.contains("U"),
            Self::Europe => s.contains("E"),
            Self::Japan => s.contains("J"),
        }
    }
}

#[derive(Debug)]
pub struct Rom {
    name: String,
    number: Option<u32>,
    tags: HashSet<String>,
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    locale: Vec<Locale>,

    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    bad_dumps: bool,
}

fn all_files<P: AsRef<Path>>(path: P) -> Result<ReadDir> {
    std::fs::read_dir(path).map_err(|_| Error::IoError)
}

fn parse_rom(file_name: &str) -> Rom {
    lazy_static! {
        static ref PREFIX: Regex = Regex::new(r"^(\d{4,}) - ").unwrap();
        static ref TAGS: Regex = Regex::new(r"(?i)\(([^)]+)\)|\[([^\]]+)\]").unwrap();
        static ref EXTENSION: Regex = Regex::new(r"\s+(\.\w+)$").unwrap();
    };

    let mut number = None;
    let file_name = PREFIX.replace(file_name, |caps: &Captures| {
        if let Some(m) = caps.get(1) {
            number = Some(u32::from_str(m.as_str()).unwrap());
        }
        Cow::Owned(String::new())
    });

    let mut tags = HashSet::with_capacity(20);
    let name = TAGS
        .replace_all(file_name.as_ref(), |caps: &Captures| {
            if let Some(cap) = caps.get(1) {
                tags.insert(cap.as_str().to_string());
            }
            ""
        })
        .to_string();

    let name = EXTENSION
        .replace(name.as_str(), |caps: &Captures| {
            caps.get(1).unwrap().as_str().to_string()
        })
        .to_string();

    Rom { number, tags, name }
}

fn locale_matches(file: &File, locales: &[Locale]) -> bool {
    locales
        .iter()
        .any(|locale| file.rom.tags.iter().any(|tag| locale.represented_by(tag)))
}

fn bad_dump_ok(file: &File, allow_bad_dumps: bool) -> bool {
    allow_bad_dumps || !file.rom.tags.contains("b")
}

#[derive(Debug)]
struct File {
    entry: DirEntry,
    rom: Rom,
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    log::info!("{:?}", &args);

    all_files(&args.path)
        .unwrap()
        .map(|f| f.unwrap())
        .map(|entry| {
            let rom = parse_rom(entry.path().file_name().unwrap().to_str().unwrap());
            File { entry, rom }
        })
        .filter(|file| locale_matches(file, args.locale.as_slice()))
        .filter(|file| bad_dump_ok(file, args.bad_dumps))
        .for_each(|f| log::info!("{:#?}", f.rom));
}
