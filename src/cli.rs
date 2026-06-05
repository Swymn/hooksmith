use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(name = "hooksmith", about = "Git hooks manager", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Installing hooks defined inside .hooksmith.toml into .git/hooks
    Init,

    /// Add command to a hook
    Add {
        /// Name of the hook
        hook: String,
        /// Command to execute
        command: String,
    },

    /// Remove a hook
    Remove { hook: String },

    /// Execute commands of a specific hook
    Run { hook: String },

    /// Display current config
    Status,

    #[command(hide = true)]
    Completions {
        /// Shell cible
        #[arg(value_enum)]
        shell: Shell,
    },
}
