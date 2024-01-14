use self::commands::Commands;

mod add;
pub mod commands;
mod common;
mod run;
pub mod types;

use self::add::add;
use self::run::run;

pub fn match_command(command: &Commands) {
    match command {
        Commands::Run { swarm_name } => run(swarm_name.clone()),
        Commands::Add => add(),
    }
}
