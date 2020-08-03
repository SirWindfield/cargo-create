use crate::{
    args::Args,
    config::user::UserConfig,
    git::{clone_into_folder, parse_to_git_url},
    providers::VariableProvider,
};
use anyhow::Result;
use log::info;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use tera::{Context, Tera};
use walkdir::WalkDir;
use std::collections::HashMap;

pub fn run(args: &Args, _user_config: &UserConfig, repo_path: impl AsRef<Path>) -> Result<()> {
    // Create the template engine and context.
    let mut ctx = Context::new();
    // Insert project and and enabled features.
    ctx.insert("project_name", &args.name);
    if let Some(enabled_features) = &args.features {
        let mut bool_map = HashMap::with_capacity(enabled_features.len());
        for enabled_feature in enabled_features {
            bool_map.insert(enabled_feature, true);
        }
        ctx.insert("features", &bool_map);
    }

    let mut providers = Vec::new();
    for variable_provider in inventory::iter::<&dyn VariableProvider> {
        providers.push(variable_provider);
    }
    providers.sort_by_key(|p| p.priority());

    for provider in providers {
        provider.populate(&mut ctx);
    }

    for entry in WalkDir::new(repo_path.as_ref())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let filename = entry.file_name().to_string_lossy();
        if filename.ends_with(".tera") {
            let raw_template = fs::read_to_string(entry.path())?;
            let rendered = Tera::one_off(&raw_template, &ctx, false)?;
            fs::write(entry.path(), rendered)?;

            let relative_to_repo_path = entry.path().strip_prefix(repo_path.as_ref())?;
            let filename_without_tera_ext = relative_to_repo_path
                .file_stem()
                .expect("failed to get file stem from .tera file");
            let parent_of_tera_file = relative_to_repo_path
                .parent()
                .expect("failed to parent of relative file path");
            let new_file_path = repo_path
                .as_ref()
                .join(parent_of_tera_file.join(filename_without_tera_ext));

            fs::rename(entry.path(), new_file_path)?;
        }
    }

    Ok(())
}
