use anyhow::Result;
use cargo_create::{args::Args, config::user::user_config_file_path, start};
use clap::Clap;
use std::process::exit;

fn run() -> Result<()> {
    let args: Args = Args::parse();
    let user_config_path =
        user_config_file_path().expect("failed to resolve user config file path");

    if args.config_path {
        println!("{}", user_config_path.display());
        Ok(())
    } else {
        start(args, user_config_path)
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("An error occurred during execution: {:?}", e);
        exit(1);
    }
}
