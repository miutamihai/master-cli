use clap::Subcommand;
use crate::git::commands::Commands as GitCommands;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about="Git related commands")]
    #[command(subcommand)]
    Git(GitCommands),
    #[command(about="Change config option")]
    Config {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        value: String
    }
}
