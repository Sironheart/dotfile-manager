use anyhow::Result;
use clap::{Arg, command};
use dotfile::DotfileSetup;
use std::path::PathBuf;

const ARGUMENT_NAME: &str = "path";

fn main() -> Result<()> {
    let matches = command!().arg(Arg::new(ARGUMENT_NAME)).get_matches();

    let path = match matches.get_one::<String>(ARGUMENT_NAME) {
        Some(str) => PathBuf::from(str).canonicalize()?,
        None => todo!(),
    };

    let _ = core::setup(&path, vec![Box::new(DotfileSetup {})]);

    Ok(())
}
