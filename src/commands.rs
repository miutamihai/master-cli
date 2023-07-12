use clap::Subcommand;
use crate::git::commands::Commands as GitCommands;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(subcommand)]
    Git(GitCommands)
}
