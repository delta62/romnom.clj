use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::{collections::HashSet, str::FromStr};

#[derive(Clone, Copy, Hash, Debug, clap::ValueEnum, PartialEq, Eq)]
pub enum Locale {
    UnitedStates,
    Europe,
    Japan,
}

impl Locale {
    pub fn represented_by(&self, s: &str) -> bool {
        match self {
            Self::UnitedStates => s.contains('U'),
            Self::Europe => s.contains('E'),
            Self::Japan => s.contains('J'),
        }
    }
}

#[derive(Debug)]
pub struct Rom {
    pub name: String,
    pub tags: HashSet<String>,
}

impl FromStr for Rom {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PREFIX: Regex = Regex::new(r"^\d{4,} - ").unwrap();
            static ref TAGS: Regex = Regex::new(r"(?i)\(([^)]+)\)|\[([^\]]+)\]").unwrap();
            static ref EXTENSION: Regex = Regex::new(r"\s+(\.\w+)$").unwrap();
        };

        let file_name = PREFIX.replace(s, "");

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

        Ok(Rom { tags, name })
    }
}
