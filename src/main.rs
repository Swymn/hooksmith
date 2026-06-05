use std::{path::PathBuf, process};

use clap::Parser;

use crate::{cli::{Cli, Commands}, config::Config};

pub mod config;
pub mod error;
pub mod cli;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run() -> error::Result<()> {
    let cli = Cli::parse();
    let git_root = PathBuf::from("./");
    let mut config = Config::load(&git_root)?;

    match cli.command {
        Commands::Init => {
            println!("✓ Hooksmith initialized!");
        },
        Commands::Add { .. } => {
            println!("✓ Command added.");
        },
        Commands::Run { .. } => {
            unimplemented!("Running hook");
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
    }

    Ok(())
}
