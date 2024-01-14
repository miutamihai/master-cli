use self::commands::Commands;

pub mod commands;
mod handlers;

use self::handlers::add::add;
use self::handlers::run::run;

pub fn match_command(command: &Commands) {
    match command {
        Commands::Run { command } => run(command.clone()),
        Commands::Add => add(),
    }
}
