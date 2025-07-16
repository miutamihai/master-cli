use super::git::commands::Commands as GitCommands;
use super::profile::commands::Commands as ProfileCommands;
use super::term::commands::Commands as TermCommands;
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
    #[command(about = "Commands related to launching terminal sessions")]
    #[command(subcommand)]
    Term(TermCommands),
}
