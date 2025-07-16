use crate::profile::add::add;
use crate::profile::commands::Commands;
use crate::profile::set::set;

pub fn match_command(command: &Commands) {
    match command {
        Commands::Add => {
            add();
        }
        Commands::Set { name } => {
            set(name.clone());
        }
    }
}
