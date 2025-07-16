use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Runs the given terminal swarm")]
    Run { swarm_name: String },

    #[command(about = "Adds a new terminal swarm")]
    Add,
}
