use crate::config::CONFIG_FILENAME_JEN;
use directories_next::{BaseDirs, ProjectDirs};
use serde::Deserialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// The config filename when running in release mode.
const USER_CONFIG_SIMPLE_FILENAME: &str = "config.toml";

/// The config filename when running in debug mode.
//#[cfg(debug_assertions)]
const DEBUG_USER_CONFIG_FILENAME: &str = "config.user.toml";

/// The user configuration file.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserConfig<'a> {
    /// A list of default values that will always be used to populate templates.
    #[serde(borrow)]
    pub defaults: HashMap<&'a str, &'a str>,
    /// Custom profiles that can be used as a shortcut.
    #[serde(borrow)]
    pub profiles: HashMap<&'a str, &'a str>,
}

pub fn user_config_file_path() -> Option<PathBuf> {
    if cfg!(debug_assertions) {
        return Some(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(DEBUG_USER_CONFIG_FILENAME));
    }

    if let Some(project_dirs) = ProjectDirs::from("net", "zerotask", "jen") {
        let config_path = project_dirs.config_dir().join(USER_CONFIG_SIMPLE_FILENAME);
        if !config_path.exists() {
            // Convenience lookup inside the home directory.
            let home_dir_config_path = BaseDirs::new()
                .map(|bd| bd.home_dir().join(".config").join(CONFIG_FILENAME_JEN))
                .filter(|p| p.exists());

            // However, if the home directory config file is not available, we still return
            // the platform-specific file location.
            return if home_dir_config_path.is_none() {
                Some(config_path)
            } else {
                home_dir_config_path
            };
        }

        return Some(config_path);
    }

    None
}
