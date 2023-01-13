mod args;
mod error;
mod filters;
mod fs;
mod term;
mod types;

use args::Args;
use clap::Parser;
use error::Error;
use filters::{bad_dump_ok, extension_matches, locale_matches};
use fs::{copy, read_dir};
use futures::future::try_join_all;
use std::path::Path;
use tokio::fs::DirEntry;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let args = Args::parse();
    let mut actions = read_dir(&args.path).await?;
    let mut tasks = Vec::new();

    while let Some(entry) = actions.next_entry().await.map_err(|_| Error::IoError)? {
        tasks.push(maybe_copy_file(entry, &args));
    }

    try_join_all(tasks)
        .await
        .map(|_| ())
        .map_err(|_| Error::IoError)
}

async fn maybe_copy_file(entry: DirEntry, args: &Args) -> Result<(), Error> {
    let path = entry.path();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let rom = file_name.parse().unwrap();
    let bytes = fs::stat(&path).await.map(|s| s.len()).unwrap_or_default();

    if !extension_matches(&path, args.extension.as_slice())
        | !locale_matches(&rom, args.locale.as_slice())
        | !bad_dump_ok(&rom, args.bad_dumps)
    {
        return Ok(());
    }

    term::print_rom(&rom, bytes);
    if !args.dry_run {
        let out = args.output.as_ref().unwrap().as_str();
        let inn = entry.path();
        let in_path = inn.as_path();
        let out_path = Path::new(out).join(rom.name);

        copy(in_path, &out_path).await.unwrap();
    }

    Ok(())
}
