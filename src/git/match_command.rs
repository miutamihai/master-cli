use crate::config::model::Config;
use crate::git::commands::Commands;
use crate::git::handlers::init::init;
use crate::git::handlers::pr::pr;
use crate::git::handlers::restart::restart;

pub fn match_command(command: &Commands, config: &Config) {
    match command {
        Commands::Init => {
            init(config);
        }
        Commands::PR => {
            pr();
        }
        Commands::Restart {destination, origin} => {
            restart(destination, origin);
        }
    }
}