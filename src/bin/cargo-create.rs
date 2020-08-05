use anyhow::Result;
use cargo_create::{
    args::Args,
    config::user::{user_config_file_path, UserConfig},
    output::{print_step, APPLYING_EMOJI, CLEANING_EMOJI, FILLING_EMOJI},
    start,
};
use clap::{
    derive::{FromArgMatches, IntoApp},
    AppSettings,
};
use std::process::exit;

fn run() -> Result<()> {
    // `clap` programs used as cargo subcommands need the first two args removed
    // before parsing.
    let args = std::env::args_os().skip(2);
    // The binary name has to be cargo here.
    let app = Args::into_app();
    // Since the binary name is stripped away earlier, we need to tell `clap` to not
    // assume that the name exists.
    let matches = app
        .bin_name("cargo create")
        .setting(AppSettings::NoBinaryName)
        .get_matches_from(args);
    let args: Args = Args::from_arg_matches(&matches);

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
