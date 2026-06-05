use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    config::Config,
    error::{HooksmithError, Result},
};

pub fn find_git_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;

    if !output.status.success() {
        return Err(crate::error::HooksmithError::NoGitRepo);
    }

    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(PathBuf::from(path))
}

pub fn install_hook(git_root: &Path, hook_name: &str) -> Result<()> {
    let hooks_dir = git_root.join(".git/hooks");
    fs::create_dir_all(&hooks_dir)?;

    let hook_path = hooks_dir.join(hook_name);

    // Create bash script for hook
    let script = format!("#!/bin/sh\nhooksmith run {hook_name}\n");
    fs::write(&hook_path, script)?;

    // Change permission to make hook executable (required by git)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut perms = fs::metadata(&hook_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook_path, perms)?;
    }

    println!("✓ Hook '{hook_name}' installed.");

    Ok(())
}

pub fn run_hook(hook_name: &str, config: &Config) -> Result<()> {
    let Some(hook_config) = config.hooks.get(hook_name) else {
        return Ok(());
    };

    for command in &hook_config.commands {
        println!("▶ {command}");

        let status = Command::new("sh").arg("-c").arg(command).status()?;

        if !status.success() {
            return Err(HooksmithError::CommandFailed {
                command: command.clone(),
                code: status.code().unwrap_or(1),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config::HookConfig;

    use super::*;

    fn config_with_commands(hook: &str, commands: &[&str]) -> Config {
        let mut config = Config::default();
        config.hooks.insert(
            hook.to_string(),
            HookConfig {
                commands: commands.iter().map(|s| s.to_string()).collect(),
            },
        );

        config
    }

    #[test]
    fn should_create_hook_file_in_git_hooks_dir() {
        // GIVEN a hook
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(&dir.path().join(".git/hooks")).unwrap();

        // WHEN installing the hook
        install_hook(dir.path(), "pre-commit").unwrap();

        // THEN the file should be created
        let hook_path = dir.path().join(".git/hooks/pre-commit");
        assert!(hook_path.exists());
    }

    #[test]
    fn should_write_correct_script_content() {
        // GIVEN a hook
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(&dir.path().join(".git/hooks")).unwrap();

        // WHEN installing the hook with specific content
        install_hook(dir.path(), "pre-push").unwrap();

        // THEN the hook should be installed with specific content
        let content = fs::read_to_string(dir.path().join(".git/hooks/pre-push")).unwrap();
        assert!(content.contains("hooksmith run pre-push"));
        assert!(content.starts_with("#!/bin/sh"));
    }

    #[test]
    fn should_succeed_silently_when_no_hook_configured() {
        // GIVEN config
        let config = Config::default();

        // WHEN running hook
        let result = run_hook("pre-commit", &config);

        // THEN the result should be ok
        assert!(result.is_ok());
    }

    #[test]
    fn should_succeed_when_all_commands_pass() {
        // GIVEN a config with commands
        let config = config_with_commands("pre-commit", &["true"]);

        // WHEN running hook
        let result = run_hook("pre-commit", &config);

        // THEN the result should be ok
        assert!(result.is_ok());
    }

    #[test]
    fn should_fail_when_a_command_fails() {
        // GIVEN config
        let config = config_with_commands("pre-commit", &["false"]);

        // WHEN running hook
        let result = run_hook("pre-commit", &config);

        // THEN the result should be an error
        assert!(matches!(result, Err(HooksmithError::CommandFailed { .. })));
    }

    #[test]
    fn should_stop_at_first_failing_command() {
        // GIVEN config
        let config = config_with_commands("pre-commit", &["false", "this-command-doesnt-exist"]);

        // WHEN running hook
        let result = run_hook("pre-commit", &config);

        // THEN the result should be an error
        assert!(matches!(
            result,
            Err(HooksmithError::CommandFailed { code: 1, .. })
        ));
    }

    #[test]
    fn should_run_command_for_the_correct_hook_only() {
        // GIVEN config
        let mut config = Config::default();
        config.hooks.insert(
            "pre-commit".to_string(),
            HookConfig {
                commands: vec!["true".to_string()],
            },
        );
        config.hooks.insert(
            "pre-push".to_string(),
            HookConfig {
                commands: vec!["false".to_string()],
            },
        );

        // WHEN running hook
        let result = run_hook("pre-commit", &config);

        // THEN the result should be ok
        assert!(result.is_ok());
    }
}
