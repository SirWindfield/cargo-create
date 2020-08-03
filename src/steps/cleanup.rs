use crate::{
    args::Args,
    config::{template::TemplateConfig, user::UserConfig, CONFIG_FILENAME_JEN},
};
use anyhow::Result;
use log::warn;
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

pub fn run(
    template_repo_dir: impl AsRef<Path>,
) -> Result<()> {
    let template_config_file_path = template_repo_dir.as_ref().join(CONFIG_FILENAME_JEN);
    fs::remove_file(template_config_file_path)?;

    Ok(())
}
