//! Step 1: Clone repository.

use crate::{
    args::Args,
    config::user::UserConfig,
    git::{clone_into_folder, parse_to_git_url},
};
use anyhow::Result;
use std::{env, path::PathBuf};
use std::process::exit;

pub fn run(args: &Args, user_config: &UserConfig) -> Result<PathBuf> {
    // `clap` does make it actually impossible (I think) to have an Option that is
    // None here.
    let repository_url = args
        .profile
        .as_ref()
        .map(|profile_name| user_config.profiles.get(profile_name.as_str()))
        .flatten()
        .or(args.git.as_deref().as_ref())
        .map(|&url| parse_to_git_url(url));
    // Still, better safe than sorry...
    if let Some(repository_url) = repository_url {
        // Detect the current working directory and clone into a folder.
        let repository_dir_path = env::current_dir()?.join(&args.name);
        // Check if the folder already exists.
        if repository_dir_path.exists() {
            eprintln!("A folder named {:?} already exists!", &args.name);
            exit(1);
        }
        clone_into_folder(&repository_url, &repository_dir_path)?;

        return Ok(repository_dir_path);
    } else {
        unreachable!();
    }
}
