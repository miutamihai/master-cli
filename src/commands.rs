use clap::Subcommand;
use crate::git_commands::{GitCommands};

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(subcommand)]
    Git(GitCommands)
}
