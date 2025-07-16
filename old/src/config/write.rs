use crate::common::exit_with_errors::exit_with_errors;
use crate::config::config_path::config_path;
use crate::config::model::Config;
use log::info;
use std::fs;
use toml::to_string;

pub fn write(config: Config, mut message: Option<String>) {
    let content = to_string(&config).unwrap();
    let success_message = message.get_or_insert("Config updated successfully".to_string());

    match fs::write(config_path(), content) {
        Ok(_) => {
            info!("{}", success_message)
        }
        Err(error) => exit_with_errors(format!("Failed to write config change: {}", error)),
    }
}
