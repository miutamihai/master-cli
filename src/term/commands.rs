use clap::Subcommand;

use super::swarm::commands::Commands as SwarmCommands;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Runs the given command in a new terminal")]
    Run { command: String },
    #[command(about = "Terminal swarm related commands")]
    #[command(subcommand)]
    Swarm(SwarmCommands),
}
