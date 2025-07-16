use clap::Parser;

use crate::cli::Cli;
use crate::commands::Commands::{Config, Git, Profile, Term};
use crate::config::ensure_default::ensure_default;
use crate::setup_logger::setup_logger;

mod cli;
mod commands;
mod common;
mod config;
mod embedded;
mod git;
mod profile;
mod setup_logger;
mod term;

fn main() {
    let cli = Cli::parse();

    setup_logger();
    ensure_default();

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
        Term(command) => {
            term::match_command::match_command(command);
        }
    }
}
