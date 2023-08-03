use crate::common::run::run;
use crate::git::handlers::restart::get_commands::get_commands;

mod build_input;
mod get_args;
mod get_commands;

pub fn restart(destination: &String, origin: &String) {
    get_commands(destination, origin)
        .into_iter()
        .flat_map(run)
        .collect()
}
