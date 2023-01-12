use crate::types::Rom;
use console::{style, Term};
use lazy_static::lazy_static;
use regex::Regex;

fn is_language(tag: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[ABCDEFGHIJKRSUW$]+|M\d").unwrap();
    }

    RE.is_match(tag)
}

fn is_version(tag: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^v\d+\.\d+$").unwrap();
    }

    RE.is_match(tag)
}

fn color_tag(tag: &str) -> (u8, String) {
    let order: u8;
    let t = style(tag);
    let tag = if is_language(tag) {
        order = 0;
        t.green()
    } else if is_version(tag) {
        order = 1;
        t.yellow()
    } else {
        order = 2;
        t
    };

    (order, format!("[{}]", tag))
}

fn format_size(size: u64) -> String {
    let num: f64;
    let suffix: &str;
    if size > 1_000_000_000 {
        num = size as f64 / 1_000_000_000.0;
        suffix = "GB";
    } else if size > 1_000_000 {
        num = size as f64 / 1_000_000.0;
        suffix = "MB";
    } else if size > 1_000 {
        num = size as f64 / 1_000.0;
        suffix = "kB";
    } else {
        num = size as f64;
        return format!("{}b", num);
    }

    format!("{:.1}{}", num, suffix)
}

pub fn print_rom(rom: &Rom, size: u64) {
    let mut tags = rom
        .tags
        .iter()
        .map(|s| s.as_str())
        .map(color_tag)
        .collect::<Vec<_>>();
    tags.sort_by(|a, b| a.0.cmp(&b.0));

    let tags: String = tags.into_iter().map(|(_, x)| x).collect();
    print!("{} ", style("+").bold().green());
    print!("{}", tags);
    // Format width doesn't work with ANSI codes in the string... just do it manually.
    Term::stdout().move_cursor_left(999999).unwrap();
    Term::stdout().move_cursor_right(15).unwrap();
    println!("{} ({})", rom.name, style(format_size(size)).blue());
}
