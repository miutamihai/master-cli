use crate::common::exit_with_errors::exit_with_errors;
use crate::common::run::Input;

pub fn build_input(args: Vec<String>) -> Input {
    Input {
        cmd: String::from("git"),
        args,
        on_done: None,
        on_error: Some(|error| exit_with_errors(error)),
    }
}
