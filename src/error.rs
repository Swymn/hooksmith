use std::{io, path, result};

#[derive(Debug, thiserror::Error)]
pub enum HooksmithError {
    #[error("Git repo not found inside this folder.")]
    NoGitRepo,

    #[error("Config file not found: {0}")]
    ConfigNotFound(path::PathBuf),

    #[error("Unable to parse toml file.")]
    TomlParse(#[from] toml::de::Error),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Unknown Hook ({0})")]
    UnknownHook(String)
}

pub type Result<T> = result::Result<T, HooksmithError>;