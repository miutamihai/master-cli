use crate::profile::commands::Commands;
use crate::profile::handlers::add::add;
use crate::profile::handlers::set::set;

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
