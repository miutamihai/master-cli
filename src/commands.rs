use crate::git::commands::Commands as GitCommands;
use crate::profile::commands::Commands as ProfileCommands;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Git related commands")]
    #[command(subcommand)]
    Git(GitCommands),
    #[command(about = "Profile related commands")]
    #[command(subcommand)]
    Profile(ProfileCommands),
    #[command(about = "Change config option")]
    Config {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        value: Option<String>,
    },
}
