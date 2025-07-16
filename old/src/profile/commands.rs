use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Adds a new profile")]
    Add,
    #[command(about = "Sets the current profile")]
    Set { name: String },
}
