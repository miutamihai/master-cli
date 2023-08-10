use crate::common::run::{run, Input};

pub fn initialize_repo() {
    let input = Input {
        cmd: "git".to_string(),
        args: vec!["init".to_string()],
        on_done: None,
        on_error: None,
    };

    run(input).ok();
}
