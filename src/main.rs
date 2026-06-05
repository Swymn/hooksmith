use std::{io, process};

use clap::{CommandFactory, Parser};
use clap_complete::generate;

use crate::{
    cli::{Cli, Commands},
    config::Config,
    error::HooksmithError,
    hooks::find_git_root,
};

pub mod cli;
pub mod config;
pub mod error;
pub mod hooks;

fn main() {
    match run() {
        Err(HooksmithError::CommandFailed { code, .. }) => {
            process::exit(code);
        }
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
        _ => {}
    }
}

fn run() -> error::Result<()> {
    let cli = Cli::parse();
    let git_root = find_git_root()?;
    let mut config = Config::load(&git_root)?;

    match cli.command {
        Commands::Init => {
            for hook_name in config.hooks.keys() {
                hooks::install_hook(&git_root, hook_name)?;
            }
            println!("✓ Hooksmith initialized!");
        }
        Commands::Add { hook, command } => {
            config.add_command(&hook, command)?;
            config.save(&git_root)?;
            hooks::install_hook(&git_root, &hook)?;
            println!("✓ Command added.");
        }
        Commands::Remove { hook } => {
            config.remove_hook(&hook)?;
            config.save(&git_root)?;
            hooks::remove_hook(&git_root, &hook)?;
            println!("✓ Hook removed.");
        }
        Commands::Run { hook } => {
            hooks::run_hook(&hook, &config)?;
        }
        Commands::Status => {
            if config.hooks.is_empty() {
                println!("No hooks configured.");
            } else {
                for (hook, cfg) in &config.hooks {
                    println!("[{hook}]");
                    for cmd in &cfg.commands {
                        println!("\t- {cmd}");
                    }
                }
            }
        }
        Commands::Completions { shell } => {
            // Génère le script sur stdout — l'utilisateur redirige lui-même
            // vers son fichier de config shell
            generate(shell, &mut Cli::command(), "hooksmith", &mut io::stdout());
        }
    }

    Ok(())
}
