use super::build_input::build_input;
use super::get_args::get_args;
use crate::common::run::Input;

pub fn get_commands(destination: &String, origin: &String) -> Vec<Input> {
    get_args(destination, origin)
        .into_iter()
        .map(build_input)
        .collect()
}
