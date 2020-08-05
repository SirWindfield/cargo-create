//! Step 1: Clone repository.

use crate::{
    args::Args,
    config::user::UserConfig,
    git::{clone_into_folder, parse_to_git_url},
};
use anyhow::Result;
use std::{env, path::PathBuf, process::exit};

// Changing to `or_else` does mean that the function owns the reference and thus
// fails on compilation.
#[allow(clippy::or_fun_call)]
pub fn run(args: &Args, user_config: &UserConfig) -> Result<PathBuf> {
    // `clap` does make it actually impossible (I think) to have an Option that is
    // None here.
    let repository_url = args
        .profile
        .as_ref()
        .map(|profile_name| {
            if let Some(profiles) = &user_config.profiles {
                profiles.get(profile_name.as_str())
            } else {
                None
            }
        })
        .flatten()
        .or(args.git.as_deref().as_ref())
        .map(|&url| parse_to_git_url(url));
    // Still, better safe than sorry...
    if let Some(repository_url) = repository_url {
        // Detect the current working directory and clone into a folder.
        let repository_dir_path = env::current_dir()?.join(&args.name.as_ref().unwrap());
        // Check if the folder already exists.
        if repository_dir_path.exists() {
            eprintln!(
                "A folder named {:?} already exists!",
                &args.name.as_ref().unwrap()
            );
            exit(1);
        }
        clone_into_folder(&repository_url, &args.branch.as_ref(), &repository_dir_path)?;

        Ok(repository_dir_path)
    } else {
        unreachable!();
    }
}
