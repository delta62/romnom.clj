use crate::types::{File, Locale};

pub fn locale_matches(file: &File, locales: &[Locale]) -> bool {
    locales
        .iter()
        .any(|locale| file.rom.tags.iter().any(|tag| locale.represented_by(tag)))
}

pub fn bad_dump_ok(file: &File, allow_bad_dumps: bool) -> bool {
    allow_bad_dumps || !file.rom.tags.contains("b")
}
