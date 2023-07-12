mod commands;
mod cli;
mod git_commands;

use clap::Parser;
use crate::cli::Cli;
use crate::commands::Commands::Git;
use crate::git_commands::GitCommands;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Git(git_command) => {
            match git_command {
                GitCommands::Init => {
                    println!("running git init")
                },
                GitCommands::PR => {
                    println!("creating github PR")
                }
            }
        }
    }
}
