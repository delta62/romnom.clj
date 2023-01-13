mod args;
mod error;
mod filters;
mod fs;
mod term;
mod types;

use std::path::Path;

use args::Args;
use clap::Parser;
use error::Error;
use filters::{bad_dump_ok, extension_matches, locale_matches};
use fs::{copy, read_dir};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let args = Args::parse();
    let mut actions = read_dir(&args.path).await?;

    while let Some(entry) = actions.next_entry().await.map_err(|_| Error::IoError)? {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let rom = file_name.parse().unwrap();
        let bytes = fs::stat(&path).await.map(|s| s.len()).unwrap_or_default();

        if !extension_matches(&path, args.extension.as_slice())
            | !locale_matches(&rom, args.locale.as_slice())
            | !bad_dump_ok(&rom, args.bad_dumps)
        {
            continue;
        }

        term::print_rom(&rom, bytes);
        if !args.dry_run {
            let output_path = Path::new(args.output.as_ref().unwrap().as_str()).join(rom.name);
            copy(entry.path().as_path(), &output_path).await.unwrap();
        }
    }

    Ok(())
}
