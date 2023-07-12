use crate::git::commands::Commands;
use crate::git::handlers::init::init;
use crate::git::handlers::pr::pr;

pub fn match_command(command: &Commands) {
    match command {
        Commands::Init => {
            init();
        }
        Commands::PR => {
            pr();
        }
    }
}