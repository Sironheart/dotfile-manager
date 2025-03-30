mod dfm;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

fn main() -> Result<()> {
    let cli = Cli::parse();

    dfm::setup(cli.path)
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(required = true)]
    path: PathBuf,
}
