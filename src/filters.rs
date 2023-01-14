use crate::types::{Locale, Rom};
use std::path::Path;

pub fn locale_matches(rom: &Rom, locales: &[Locale]) -> bool {
    locales.is_empty()
        || locales
            .iter()
            .any(|locale| rom.tags.iter().any(|tag| locale.represented_by(tag)))
}

pub fn bad_dump_ok(rom: &Rom, allow_bad_dumps: bool) -> bool {
    allow_bad_dumps || !rom.tags.contains("b")
}

pub fn extension_matches<P: AsRef<Path>, T: AsRef<str>>(path: P, extensions: &[T]) -> bool {
    if extensions.is_empty() {
        return true;
    }

    let os_path = path.as_ref().extension();

    if let Some(path) = os_path {
        let path = path.to_str().unwrap();
        extensions.iter().any(|e| path == e.as_ref())
    } else {
        false
    }
}

pub fn video_ok(rom: &Rom, allow_videos: bool) -> bool {
    allow_videos || !rom.tags.contains("Video")
}
