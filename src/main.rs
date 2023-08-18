use clap::Parser;

use crate::cli::Cli;
use crate::commands::Commands::Profile;
use crate::commands::Commands::{Config, Git};
use crate::config::get_or_default::get_or_default;
use crate::setup_logger::setup_logger;

mod cli;
mod commands;
mod common;
mod config;
mod embedded;
mod git;
mod profile;
mod setup_logger;

fn main() {
    setup_logger();

    let cli = Cli::parse();
    let config = get_or_default();

    match &cli.command {
        Git(command) => {
            git::match_command::match_command(command, config);
        }

        Config { name, value } => {
            config::handle::handle(name, value, config);
        }
        Profile(command) => {
            profile::match_command::match_command(command);
        }
    }
}
