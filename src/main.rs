mod commands;
mod cli;
mod git;
mod setup_logger;
mod common;

use clap::Parser;
use crate::cli::Cli;
use crate::commands::Commands::Git;
use crate::setup_logger::setup_logger;

fn main() {
    setup_logger();

    let cli = Cli::parse();

    match &cli.command {
        Git(git_command) => {
            git::match_command::match_command(git_command);
        }
    }
}
