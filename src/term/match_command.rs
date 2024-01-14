use super::commands::Commands;
use super::handlers::run::run;
use super::handlers::swarm;

pub fn match_command(command: &Commands) {
    match command {
        Commands::Run { command } => {
            run(command.clone());
        }
        Commands::Swarm(command) => swarm::match_command(command),
    }
}
