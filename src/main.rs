mod args;
mod error;
mod filters;
mod fs;
mod term;
mod types;

use args::Args;
use clap::Parser;
use error::{Error, Result};
use filters::{bad_dump_ok, extension_matches, locale_matches, video_ok};
use fs::{copy, read_dir};
use futures::future::try_join_all;
use std::{path::Path, time::Instant};
use tokio::fs::DirEntry;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env_logger::init();

    let start_time = Instant::now();
    let args = Args::parse();
    let mut actions = read_dir(&args.path).await?;
    let mut tasks = Vec::new();

    while let Some(entry) = actions.next_entry().await.map_err(Error::IoError)? {
        tasks.push(maybe_copy_file(entry, &args));
    }

    let (processed, ignored): (Vec<bool>, Vec<bool>) =
        try_join_all(tasks).await?.into_iter().partition(|x| *x);

    let duration = Instant::now().duration_since(start_time);
    term::print_duration(processed.len(), ignored.len(), duration);

    Ok(())
}

async fn maybe_copy_file(entry: DirEntry, args: &Args) -> Result<bool> {
    let path = entry.path();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let rom = file_name.parse().unwrap();
    let bytes = fs::stat(&path).await.map(|s| s.len()).unwrap_or_default();

    if !extension_matches(&path, args.extension.as_slice())
        | !locale_matches(&rom, args.locale.as_slice())
        | !bad_dump_ok(&rom, args.bad_dumps)
        | !video_ok(&rom, args.videos)
    {
        return Ok(false);
    }

    term::print_rom(&rom, bytes);
    if !args.dry_run {
        let out = args.output.as_ref().unwrap().as_str();
        let inn = entry.path();
        let in_path = inn.as_path();
        let out_path = Path::new(out).join(rom.name);

        copy(in_path, &out_path).await?;
    }

    Ok(true)
}
