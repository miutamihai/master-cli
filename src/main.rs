mod cli;
mod commands;
mod common;
mod config;
mod embedded;
mod git;
mod setup_logger;

use crate::cli::Cli;
use crate::commands::Commands::{Config, Git};
use crate::config::get_or_default::get_or_default;
use crate::setup_logger::setup_logger;
use clap::Parser;

fn main() {
    setup_logger();

    let cli = Cli::parse();
    let config = get_or_default();

    match &cli.command {
        Git(git_command) => {
            git::match_command::match_command(git_command, config);
        }

        Config { name, value } => {
            config::handle::handle(name, value, config);
        }
    }
}
