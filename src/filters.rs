use crate::types::{File, Locale};

pub fn locale_matches(file: &File, locales: &[Locale]) -> bool {
    locales.is_empty()
        || locales
            .iter()
            .any(|locale| file.rom.tags.iter().any(|tag| locale.represented_by(tag)))
}

pub fn bad_dump_ok(file: &File, allow_bad_dumps: bool) -> bool {
    allow_bad_dumps || !file.rom.tags.contains("b")
}

pub fn extension_matches(file: &File, extensions: &[&str]) -> bool {
    if extensions.is_empty() {
        return true;
    }

    let path = file.entry.path();
    let os_path = path.as_path().extension();

    if let Some(path) = os_path {
        extensions.contains(&path.to_str().unwrap())
    } else {
        false
    }
}
