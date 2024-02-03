use log::{error, info};

use crate::common::exit_with_errors::exit_with_errors;
use crate::embedded::settings::get::get;

use std::path::PathBuf;
use std::process::Command;

use super::swarm::types::SwarmType;

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

fn get_window_args(command_string: &String) -> Vec<String> {
    vec![
        "--single-instance".to_string(),
        "--hold".to_string(),
        "sh".to_string(),
        "-c".to_string(),
        command_string.clone(),
    ]
}

fn get_tab_args(command_string: &String) -> Vec<String> {
    vec![
        "--single-instance".to_string(),
        "@".to_string(),
        "launch".to_string(),
        "--type".to_string(),
        "tab".to_string(),
        "sh".to_string(),
        "-c".to_string(),
        command_string.clone(),
    ]
}

pub fn run_command(
    command_string: String,
    working_directory: Option<String>,
    swarm_type: SwarmType,
) {
    let args = match swarm_type {
        SwarmType::Window => get_window_args(&command_string),
        SwarmType::Tab => get_tab_args(&command_string),
    };

    // TODO: adapt this for other terminal types
    let mut command = Command::new("kitty");

    if let Some(working_directory) = get_working_directory(working_directory) {
        let absolute_path = PathBuf::from(working_directory.clone());

        if let Ok(absolute_path) = absolute_path.canonicalize() {
            command.current_dir(absolute_path);
        } else {
            exit_with_errors(format!("Path invalid: {}", working_directory));
        }
    }

    match command.args(args).spawn() {
        Ok(child) => {
            info!("Started child process with pid: {}", child.id())
        }
        Err(error) => {
            error!("Failed to start child process: {}", error)
        }
    }
}
