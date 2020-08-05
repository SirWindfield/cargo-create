//! Step 2: Apply features and delete all unneeded files.

use crate::{
    args::Args,
    config::{template::TemplateConfig, user::UserConfig, CONFIG_FILENAME_JEN},
};
use anyhow::Result;
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

pub fn run(
    args: &Args,
    _user_config: &UserConfig,
    template_repo_dir: impl AsRef<Path>,
) -> Result<()> {
    // Read the repository configuration file.
    let template_config_file_path = template_repo_dir.as_ref().join(CONFIG_FILENAME_JEN);
    if template_config_file_path.exists() {
        let mut content = String::new();
        let mut file = File::open(template_config_file_path)?;
        file.read_to_string(&mut content)?;
        let template_config: TemplateConfig = toml::from_str(&content)?;

        // Only check if the user actually specified some features that he wants.
        let files_to_delete: Vec<&str> = if let Some(features) = &args.features {
            let slice_of_features = &template_config
                .features
                .iter()
                .map(|feat| feat)
                .collect::<Vec<_>>();

            let mut resolved_features = Vec::new();
            features
                .iter()
                .map(|feature_name| template_config.feature(feature_name))
                .flatten()
                .map(|feat| feat.get_features_for_feature(slice_of_features.as_slice()))
                .for_each(|feat| {
                    resolved_features.extend_from_slice(&feat);
                });

            template_config
                .features
                .iter()
                .filter(|feat| {
                    !resolved_features
                        .iter()
                        .any(|feat_name| feat.name == feat_name.name)
                })
                .map(|feat| feat.files.as_ref())
                .flatten()
                .flatten()
                .copied()
                .collect::<Vec<_>>()
        } else {
            template_config
                .features
                .iter()
                .map(|feat| &feat.files)
                .flatten()
                .flatten()
                .copied()
                .collect::<Vec<_>>()
        };

        for file in files_to_delete {
            fs::remove_file(template_repo_dir.as_ref().join(file))?;
        }
    }

    Ok(())
}
