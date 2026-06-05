use std::{collections::HashMap, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::error::{HooksmithError, Result};

const CONFIG_FILE: &str = ".hooksmith.toml";
const VALID_HOOKS: &[&str] = &["pre-commit", "commit-msg", "pre-push", "post-commit"];

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
    /// Load the configuration from .hooksmith.toml file of the project directory
    pub fn load(project_root_dir: &Path) -> Result<Self> {
        let path = project_root_dir.join(CONFIG_FILE);

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Add a command for the given hook
    pub fn add_command(&mut self, hook: &str, command: String) -> Result<()> {
        if !VALID_HOOKS.contains(&hook) {
            return Err(HooksmithError::UnknownHook(hook.to_string()));
        }

        self.hooks
            .entry(hook.to_string())
            .or_default()
            .commands
            .push(command);

        Ok(())
    }

    /// Remove a hook
    pub fn remove_hook(&mut self, hook: &str) -> Result<()> {
        if !VALID_HOOKS.contains(&hook) {
            return Err(HooksmithError::UnknownHook(hook.to_string()));
        }

        if let None = self.hooks.remove(hook) {
            return Err(HooksmithError::MissingHook(hook.to_string()));
        }

        Ok(())
    }

    /// Should save the configuration into .hooksmithfile.toml from the project
    pub fn save(&self, project_root: &Path) -> Result<()> {
        let path = project_root.join(CONFIG_FILE);
        let contents = toml::to_string_pretty(self).unwrap();
        fs::write(path, contents)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        path::{self, PathBuf},
        vec,
    };

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
    fn should_fail_on_malformed_toml() {
        let (dir, _) = write_config("ceci n'est pas du TOML valide [[[");
        let result = Config::load(dir.path());
        assert!(matches!(result, Err(HooksmithError::TomlParse(_))));
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
        assert_eq!(
            vec!["cargo fmt --check", "cargo test"],
            config.hooks["pre-commit"].commands
        );
    }

    #[test]
    fn should_add_command_to_existing_hook() {
        // GIVEN config
        let mut config = Config::default();

        // WHEN adding command to an existing hook
        config
            .add_command("pre-commit", "cargo test".to_string())
            .unwrap();
        config
            .add_command("pre-commit", "cargo clippy".to_string())
            .unwrap();

        // THEN the command should be stored inside the configuration
        let commands = &config.hooks["pre-commit"].commands;
        assert_eq!(2, commands.len());
        assert_eq!("cargo test", commands[0]);
        assert_eq!("cargo clippy", commands[1]);
    }

    #[test]
    fn should_reject_unknown_hook_name() {
        // GIVEN a config
        let mut config = Config::default();

        // WHEN adding a command with an unknown hookname
        let result = config.add_command("pre_smth", "cargo test".to_string());

        // THEN the result should be an error
        assert!(matches!(result, Err(HooksmithError::UnknownHook(_))));
    }

    #[test]
    fn should_persist_and_reload_config() {
        let dir = tempfile::tempdir().unwrap();
        // GIVEN a config
        let mut config = Config::default();

        // WHEN adding a command
        config
            .add_command("pre-commit", "cargo test".to_string())
            .unwrap();
        // AND saving the command
        config.save(dir.path()).unwrap();

        // THEN reloaded the configuration should make the new command appear
        let reloaded = Config::load(dir.path()).unwrap();
        assert_eq!(
            config.hooks["pre-commit"].commands,
            reloaded.hooks["pre-commit"].commands
        );
    }

    #[test]
    fn should_remove_hook_from_config() {
        // GIVEN a config with one hook
        let mut config = Config::default();
        config
            .add_command("pre-commit", "cargo test".to_string())
            .unwrap();

        // WHEN removing the hook
        config.remove_hook("pre-commit").unwrap();

        // THEN the config should no longer contains hook
        assert_eq!(0, config.hooks.len());
    }

    #[test]
    fn should_reject_remove_from_config_missing_hook() {
        // GIVEN a config with no hook
        let mut config = Config::default();

        // WHEN removing the hook
        let result = config.remove_hook("pre-commit");

        // THEN the result should contains an error
        assert!(matches!(result, Err(HooksmithError::MissingHook(_))));
    }
}
