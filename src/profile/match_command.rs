use crate::profile::commands::Commands;
use crate::profile::handlers::add::add;
use log::info;

pub fn match_command(command: &Commands) {
    match command {
        Commands::Add => {
            info!("Adding new profile...");
            add();
        }
        Commands::Set { name } => {
            info!("Setting {} as current profile", name)
        }
    }
}
