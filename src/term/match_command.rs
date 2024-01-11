use super::commands::Commands;
use super::handlers::run::run;

pub fn match_command(command: &Commands) {
    match command {
        Commands::Run { command } => {
            run(command.clone());
        }
    }
}
