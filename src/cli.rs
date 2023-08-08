use clap::Parser;
use crate::commands::Commands;

#[derive(Parser, Debug)]
#[command(author, version, long_about = None)]
#[command(about="CLI to handle a master maker's common tasks")]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands
}
