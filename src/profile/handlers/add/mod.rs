use log::info;

use crate::common::exit_with_errors::exit_with_errors;
use crate::profile::handlers::add::edit::edit;
use crate::profile::handlers::add::get_template::get_template;

mod edit;
mod get_template;

pub fn add() {
    let template = get_template();

    match edit(template) {
        Ok(editted) => {
            info!("Writing profile:");
            info!("{}", editted);
        }
        Err(error) => exit_with_errors(error.to_string()),
    }
}
