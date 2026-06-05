use std::{collections::HashMap, fs, path::Path};

use serde::{ Deserialize, Serialize };

use crate::error::Result;

const CONFIG_FILE: &str = ".hooksmith.toml";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub hooks: HashMap<String, HookConfig>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HookConfig {
    pub commands: Vec<String>,
}

impl Config {
    pub fn load(project_root_dir: &Path) -> Result<Self> {
        let path = project_root_dir.join(CONFIG_FILE);

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use std::{path::{self, PathBuf}, vec};

    use super::*;

    fn write_config(content: &str) -> (tempfile::TempDir, path::PathBuf) {
        let dir = tempfile::tempdir().expect("unable to create temporary folder.");
        let config_path = dir.path().join(".hooksmith.toml");
        fs::write(&config_path, content).expect("unable to write inside config file.");
        (dir, config_path)
    }

    #[test]
    fn should_return_default_config_when_folder_does_not_exist() {
        // GIVEN an unknown folder path
        let folder = PathBuf::from("./missing-folder");

        // WHEN loading the project
        let result = Config::load(&folder);

        // THEN the result should be default
        assert_eq!(0, result.unwrap().hooks.len())
    }

    #[test]
    fn should_return_default_config_when_file_is_absent() {
        // GIVEN an project with no hooksmith file
        let folder = tempfile::tempdir().unwrap();

        // WHEN loading the project
        let result = Config::load(&folder.path());

        // THEN the result should be default
        assert_eq!(0, result.unwrap().hooks.len());
    }

    #[test]
    fn should_load_hook_commands_from_existing_config() {
        // GIVEN a project with hooksmith file 
        let toml = r#"
            [hooks.pre-commit]
            commands = ["cargo fmt --check", "cargo test"]
        "#;
        let (dir, _) = write_config(toml);

        // WHEN loading the config
        let config = Config::load(&dir.path()).unwrap();

        // THEM the config should contain one hook
        assert_eq!(1, config.hooks.len());
        assert_eq!(vec!["cargo fmt --check", "cargo test"], config.hooks["pre-commit"].commands);
    }
}
