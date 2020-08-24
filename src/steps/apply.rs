//! Step 2: Apply features and delete all unneeded files as well as rename those
//! that are kept.

use crate::{
    args::Args,
    config::{
        template::{FileEntry, MoveFileEntry, TemplateConfig},
        user::UserConfig,
        CONFIG_FILENAME_JEN,
    },
};
use anyhow::Result;
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

pub type FilesToRemoveAndRename<'a> = (Vec<&'a str>, Vec<MoveFileEntry<'a>>);

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
        let files_to_operate_on: FilesToRemoveAndRename = if let Some(features) = &args.features {
            let slice_of_features = &template_config.features.iter().collect::<Vec<_>>();

            // Holds all features that have been enabled by the user. The feature have
            // already been expanded, meaning that super-features are resolved into their
            // smaller parts.
            let mut resolved_features = Vec::new();
            features
                .iter()
                .map(|feature_name| template_config.feature(feature_name))
                .flatten()
                .map(|feat| feat.get_features_for_feature(slice_of_features.as_slice()))
                .for_each(|feat| {
                    resolved_features.extend_from_slice(&feat);
                });

            // Partition the files based on whether they are included or not. Included files
            // need to be renamed. Excluded files deleted. The left vector contains all
            // excluded files, the right one all included ones.
            let files_to_operate_on: (Vec<_>, Vec<_>) =
                template_config.features.iter().partition(|feat| {
                    !resolved_features
                        .iter()
                        .any(|feat_name| feat.name == feat_name.name)
                });
            println!("partitioned: {:#?}", files_to_operate_on);

            // Collect all files to rename.
            let files_to_rename = files_to_operate_on
                .1
                .iter()
                .map(|feat| feat.files.as_ref())
                .flatten()
                .flatten()
                .copied()
                .map(|entry| match entry {
                    FileEntry::Complex(inner) => Some(inner),
                    _ => None,
                })
                .flatten()
                .collect::<Vec<_>>();

            let files_to_delete = files_to_operate_on
                .0
                .iter()
                .map(|feat| feat.files.as_ref())
                .flatten()
                .flatten()
                .copied()
                .map(|entry| match entry {
                    FileEntry::Simple(filename) => filename,
                    FileEntry::Complex(MoveFileEntry {from, ..}) => from,
                })
                .collect::<Vec<_>>();

            (files_to_delete, files_to_rename)
        } else {
            (
                template_config
                    .features
                    .iter()
                    .map(|feat| &feat.files)
                    .flatten()
                    .flatten()
                    .copied()
                    .map(|entry| match entry {
                        FileEntry::Simple(ref filename) => filename,
                        FileEntry::Complex(MoveFileEntry { from, .. }) => from,
                    })
                    .collect::<Vec<_>>(),
                Vec::new(),
            )
        };

        println!("{:#?}", &files_to_operate_on);

        // Delete all files that are not needed.
        for file in &files_to_operate_on.0 {
            fs::remove_file(template_repo_dir.as_ref().join(file))?;
        }

        // Rename files.
        for file in &files_to_operate_on.1 {
            let dir = template_repo_dir.as_ref();
            let from = &dir.join(file.from);
            let to = &dir.join(file.to);

            fs::rename(from, to)?;
        }
    }

    Ok(())
}
