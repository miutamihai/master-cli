use crate::common::run::run;
use crate::common::validate::validate;
use crate::git::handlers::restart::get_commands::get_commands;
use crate::git::handlers::restart::get_input::get_input;

mod build_input;
mod get_args;
mod get_commands;
mod get_input;

const DESTINATION_MESSAGE: &str = "What's your destination branch? (branch you want to work with)";
const ORIGIN_MESSAGE: &str = "What's your origin branch? (branch you want to rebase from)";

pub fn restart() {
    let destination = get_input(DESTINATION_MESSAGE);
    validate(destination != "", "Destination must not be empty!");
    let origin = get_input(ORIGIN_MESSAGE);
    validate(origin != "", "Origin must not be empty!");

    get_commands(destination, origin)
        .into_iter()
        .flat_map(run)
        .collect()
}
