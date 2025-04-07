use anyhow::Result;
use clap::{Arg, command};
use core::ConfigurationAdapter;
use dotfile::DotfileAdapter;
use std::path::PathBuf;

const ARGUMENT_NAME: &str = "path";

fn main() -> Result<()> {
    let matches = command!().arg(Arg::new(ARGUMENT_NAME)).get_matches();

    let path = match matches.get_one::<&str>(ARGUMENT_NAME) {
        Some(str) => PathBuf::from(str).canonicalize()?,
        None => todo!(),
    };

    let _ = core::setup(&path);
    let test = DotfileAdapter::new(path.as_path());
    if test.is_responsible() {
        println!("{:?}", &test.dotfile_configuration);
    }
    test.execute();

    Ok(())
}
