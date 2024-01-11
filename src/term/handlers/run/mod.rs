use crate::common::exit_with_errors::exit_with_errors;
use crate::common::run::{run as run_command, Input};
use crate::common::traits::command_input_from_string::CommandInputFromString;

pub fn run(command: String) {
    if let Ok(_) = Input::from_string(command.clone()) {
        // TODO: make this more generic
        let kitty_command = Input {
            cmd: "kitty".to_string(),
            args: vec![
                "--hold".to_string(),
                "sh".to_string(),
                "-c".to_string(),
                command,
            ],
            on_done: None,
            on_error: None,
        };

        run_command(kitty_command).ok();
    } else {
        exit_with_errors("Invalid command");
    }
}
