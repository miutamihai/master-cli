use super::commands::Commands;
use super::handlers::init::init;
use super::handlers::pr::pr;
use super::handlers::rebase::rebase;
use super::handlers::restart::restart;
use crate::config::validations::validate_profile_set::validate_profile_set;

pub fn match_command(command: &Commands) {
    validate_profile_set();

    match command {
        Commands::Init => {
            init();
        }
        Commands::PR => {
            pr();
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
