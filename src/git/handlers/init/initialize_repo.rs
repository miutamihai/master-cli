use crate::common::exit_with_errors::exit_with_errors;
use crate::common::run::{Input, run};

pub fn initialize_repo() {
    let input = Input {
        cmd: "git".to_string(),
        args: vec!["init".to_string()],
        on_done: None,
        on_error: Some(|error| exit_with_errors(error)),
    };

    run(input).ok();
}
