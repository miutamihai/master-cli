use crate::common::exit_with_errors::exit_with_errors;
use crate::config::config_path::config_path;
use edit::edit_file;
use log::info;

pub fn in_editor() {
    match edit_file(config_path()) {
        Ok(_) => {
            info!("Config updated!")
        }
        Err(_) => exit_with_errors("Failed to edit config"),
    }
}
