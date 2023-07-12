use clap::Parser;
use crate::commands::Commands;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands
}
