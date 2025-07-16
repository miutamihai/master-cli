use log::info;

use crate::common::exit_with_errors::exit_with_errors;
use crate::common::traits::path_as_string::PathAsString;
use crate::git::init::ssh_config::build::build;
use crate::git::init::ssh_config::write::write;

mod build;
mod write;

pub fn setup(key_path: &String) -> String {
    let content = build(key_path);

    match write(content) {
        Ok(path) => {
            info!("Wrote ssh config");

            path.to_string()
        }
        Err(error) => exit_with_errors(format!(
            "Failed to write config because: {}",
            error.to_string()
        )),
    }
}
