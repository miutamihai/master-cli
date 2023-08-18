use crate::profile::commands::Commands;
use log::info;

pub fn match_command(command: &Commands) {
    match command {
        Commands::Add => {
            info!("Adding new profile...")
        }
        Commands::Set { name } => {
            info!("Setting {} as current profile", name)
        }
    }
}
