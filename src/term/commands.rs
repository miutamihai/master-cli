use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Runs the given command in a new terminal")]
    Run { command: String },
}
