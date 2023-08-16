use log::info;

use crate::common::exit_with_errors::exit_with_errors;
use crate::git::handlers::init::ssh_config::build::build;
use crate::git::handlers::init::ssh_config::model::Type;
use crate::git::handlers::init::ssh_config::write::write;

mod build;
pub mod model;
mod write;

pub fn setup(key_path: &String, key_type: Type) {
    let content = build(key_path);

    match write(content, key_type) {
        Ok(_) => {
            info!("Wrote ssh config")
        }
        Err(error) => exit_with_errors(format!(
            "Failed to write config because: {}",
            error.to_string()
        )),
    }
}
