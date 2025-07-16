use super::commands::Commands;
use super::run::run;
use super::swarm;

pub fn match_command(command: &Commands) {
    match command {
        Commands::Run { command } => {
            run(command.clone());
        }
        Commands::Swarm(command) => swarm::match_command(command),
    }
}
