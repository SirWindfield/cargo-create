use crate::{
    args::Args,
    config::user::{user_config_file_path, UserConfig},
    output::{print_step, APPLYING_EMOJI, CLEANING_EMOJI, FILLING_EMOJI},
};
use anyhow::Result;
use clap::Clap;
use std::{fs::OpenOptions, io::Read, path::PathBuf, process::exit};

pub mod args;
pub mod config;
pub mod git;
pub mod output;
pub mod providers;
pub mod steps;

pub fn start(args: Args, user_config_path: PathBuf) -> Result<()> {
    // Read in user config file.
    let mut user_config_content = String::new();
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(user_config_path)?;
    file.read_to_string(&mut user_config_content)?;
    let user_config: UserConfig = toml::from_str(&user_config_content)?;

    // First step: Clone repository.
    print_step(1, "> Cloning template repository...");
    let repo_dir_path = crate::steps::clone::run(&args, &user_config)?;
    // Second step: Apply features and delete files that are not enabled by any
    // feature.
    print_step(2, &format!("{} Applying features...", APPLYING_EMOJI));
    crate::steps::apply::run(&args, &user_config, &repo_dir_path)?;
    // Fourth step: Create the variable context and fill out all template files.
    print_step(3, &format!("{} Filling out templates...", FILLING_EMOJI));
    crate::steps::populate::run(&args, &user_config, &repo_dir_path)?;
    // Fifth step: Cleaning up.
    print_step(4, &format!("{} Cleaning up...", CLEANING_EMOJI));
    crate::steps::cleanup::run(&repo_dir_path)?;

    Ok(())
}
