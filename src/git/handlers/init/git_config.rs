use crate::common::run::{run, Input};
use crate::git::handlers::init::error_handler::error_handler;

pub fn git_config(key: &str, value: &str) {
    let input = Input {
        cmd: "git".to_string(),
        args: vec!["config".to_string(), key.to_string(), value.to_string()],
        on_done: None,
        on_error: Some(error_handler),
    };

    run(input).ok();
}
