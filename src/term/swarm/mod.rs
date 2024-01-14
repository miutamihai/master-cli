use self::commands::Commands;

mod add;
pub mod commands;
mod run;

use self::add::add;
use self::run::run;

pub fn match_command(command: &Commands) {
    match command {
        Commands::Run { command } => run(command.clone()),
        Commands::Add => add(),
    }
}
