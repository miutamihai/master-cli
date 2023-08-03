use crate::common::run::Input;
use crate::git::handlers::restart::build_input::build_input;
use crate::git::handlers::restart::get_args::get_args;

pub fn get_commands(destination: &String, origin: &String) -> Vec<Input> {
    get_args(destination, origin)
        .into_iter()
        .map(build_input)
        .collect()
}
