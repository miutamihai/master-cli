use crate::config::model::Config;
use crate::profile::commands::Commands;
use crate::profile::handlers::add::add;
use log::info;

pub fn match_command(command: &Commands, config: Config) {
    match command {
        Commands::Add => {
            info!("Adding new profile...");
            add(config);
        }
        Commands::Set { name } => {
            info!("Setting {} as current profile", name)
        }
    }
}
