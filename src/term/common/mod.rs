use crate::common::exit_with_errors::exit_with_errors;
use crate::embedded::settings::get::get;

use std::path::PathBuf;
use std::process::Command;

fn get_working_directory(option: Option<String>) -> Option<String> {
    let default_value = get().config.default_value;

    match option {
        Some(working_directory) => {
            if working_directory == default_value {
                return None;
            } else {
                return Some(working_directory);
            }
        }
        None => None,
    }
}

pub fn run_term(command_string: String, working_directory: Option<String>) {
    let args = [
        "--hold".to_string(),
        "sh".to_string(),
        "-c".to_string(),
        command_string.clone(),
    ];

    let mut command = Command::new("kitty");

    if let Some(working_directory) = get_working_directory(working_directory) {
        let absolute_path = PathBuf::from(working_directory.clone());

        if let Ok(absolute_path) = absolute_path.canonicalize() {
            command.current_dir(absolute_path);
        } else {
            exit_with_errors(format!("Path invalid: {}", working_directory));
        }
    }

    command
        .args(args)
        .output()
        .expect(format!("Failed to run command: {}", command_string).as_str());
}
