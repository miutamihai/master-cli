use log::{error, info};

use crate::embedded::settings::get::get;
use crate::{common::exit_with_errors::exit_with_errors, config};

use std::path::PathBuf;
use std::process::Command;

use super::swarm::types::{Swarm, SwarmEnvironment, SwarmType};

fn get_working_directory(option: Option<String>) -> Option<String> {
    let default_value = get().config.default_value;

    option.filter(|working_directory| *working_directory != default_value)
}

pub fn run_command(command_string: String, swarm: Swarm) {
    let config = config::get::get();
    let swarm_inputs = config.terminal.get_swarm_input();
    let args = match swarm.swarm_type {
        SwarmType::Window => [swarm_inputs.window_arguments, vec![command_string]].concat(),
        SwarmType::Tab => [swarm_inputs.tab_arguments, vec![command_string]].concat(),
    };

    let mut command = Command::new::<String>(config.terminal.into());

    if let Some(working_directory) = get_working_directory(swarm.working_directory) {
        let absolute_path = PathBuf::from(working_directory.clone());

        if let Ok(absolute_path) = absolute_path.canonicalize() {
            command.current_dir(absolute_path);
        } else {
            exit_with_errors(format!("Path invalid: {}", working_directory));
        }
    }

    let pula = command.args(args);
    dbg!(&pula);
    match pula.spawn() {
        Ok(child) => {
            info!("Started child process with pid: {}", child.id())
        }
        Err(error) => {
            error!("Failed to start child process: {}", error)
        }
    }
}
