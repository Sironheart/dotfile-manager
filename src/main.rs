#![allow(unknown_lints)]
#![allow(clippy)]

use clap::Parser;
use core::SetupAdapter;
use dotfile::DotfileSetup;
use mac::MacSetup;
use std::path::PathBuf;

fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let os_specific_module: Box<dyn SetupAdapter> = if cfg!(target_os = "macos") {
        Box::new(MacSetup {})
    } else {
        tracing::error!("Not supported OS");
        return;
    };

    let default_modules: Vec<Box<dyn SetupAdapter>> =
        vec![Box::new(DotfileSetup {}), os_specific_module];

    let _ = core::setup(args.path.as_path(), args.force, default_modules);
}

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help(true))]
struct Args {
    path: PathBuf,

    #[arg(short, long)]
    force: bool,
}
