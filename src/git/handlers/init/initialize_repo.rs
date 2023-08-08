use crate::common::run::{run, Input};
use crate::git::handlers::init::error_handler::error_handler;

pub fn initialize_repo() {
    let input = Input {
        cmd: "git".to_string(),
        args: vec!["init".to_string()],
        on_done: None,
        on_error: Some(error_handler),
    };

    run(input).ok();
}
