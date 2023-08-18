use clap::Parser;

use crate::cli::Cli;
use crate::commands::Commands::Profile;
use crate::commands::Commands::{Config, Git};
use crate::config::ensure_default::ensure_default;
use crate::setup_logger::setup_logger;
use crate::validate_profile_set::validate_profile_set;

mod cli;
mod commands;
mod common;
mod config;
mod embedded;
mod git;
mod profile;
mod setup_logger;
mod validate_profile_set;

fn main() {
    let cli = Cli::parse();

    setup_logger();
    ensure_default();
    validate_profile_set();

    match &cli.command {
        Git(command) => {
            git::match_command::match_command(command);
        }

        Config { name, value } => {
            config::handle::handle(name, value);
        }
        Profile(command) => {
            profile::match_command::match_command(command);
        }
    }
}
