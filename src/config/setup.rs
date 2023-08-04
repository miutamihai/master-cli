use std::process::exit;
use log::error;
use crate::config::get::get;
use crate::config::model::Config;

pub fn setup() -> Config {
    match get() {
        Ok(config) => config,
        Err(error) => {
            error!("{}", error.to_string());
            exit(1)
        }
    }

}