mod commands;
mod cli;
mod git;

use clap::Parser;
use crate::cli::Cli;
use crate::commands::Commands::Git;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Git(git_command) => {
            git::match_command::match_command(git_command);
        }
    }
}
