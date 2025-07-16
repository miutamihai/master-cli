use super::commands::Commands;
use super::init::init;
use super::rebase::rebase;
use super::restart::restart;
use crate::config::validations::validate_profile_set::validate_profile_set;

pub fn match_command(command: &Commands) {
    validate_profile_set();

    match command {
        Commands::Init => {
            init();
        }
        Commands::Restart {
            destination,
            origin,
        } => {
            restart(destination, origin);
        }
        Commands::Rebase {
            origin,
            destination,
        } => rebase(destination, origin),
    }
}
