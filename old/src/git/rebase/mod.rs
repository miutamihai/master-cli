use super::rebase::get_commands::get_commands;
use crate::common::run::run;

mod build_input;
mod get_args;
mod get_commands;

pub fn rebase(destination: &String, origin: &String) {
    get_commands(destination, origin)
        .into_iter()
        .flat_map(run)
        .collect()
}
