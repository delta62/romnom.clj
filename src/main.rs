mod args;
mod error;
mod filters;
mod fs;
mod term;
mod types;

use std::path::Path;

use args::Args;
use clap::Parser;
use filters::{bad_dump_ok, extension_matches, locale_matches};
use fs::{all_files, copy};
use types::File;

fn main() {
    env_logger::init();

    let args = Args::parse();
    let actions = all_files(&args.path)
        .unwrap()
        .map(|f| f.unwrap())
        .map(|entry| {
            let rom = entry
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .parse()
                .unwrap();
            File { entry, rom }
        })
        .filter(|file| {
            extension_matches(
                file,
                args.extension
                    .iter()
                    .map(|x| x.as_str())
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
        })
        .filter(|file| locale_matches(file, args.locale.as_slice()))
        .filter(|file| bad_dump_ok(file, args.bad_dumps));

    for action in actions {
        term::print_rom(&action.rom);
        if !args.dry_run {
            let output_path =
                Path::new(args.output.as_ref().unwrap().as_str()).join(action.rom.name);
            copy(action.entry.path().as_path(), &output_path).unwrap();
        }
    }
}
