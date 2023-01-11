mod args;
mod error;
mod filters;
mod fs;
mod types;

use args::Args;
use clap::Parser;
use filters::{bad_dump_ok, locale_matches};
use fs::all_files;
use types::File;

fn main() {
    env_logger::init();

    let args = Args::parse();
    log::info!("{:?}", &args);

    all_files(&args.path)
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
        .filter(|file| locale_matches(file, args.locale.as_slice()))
        .filter(|file| bad_dump_ok(file, args.bad_dumps))
        .for_each(|f| log::info!("{:#?}", f.rom));
}
