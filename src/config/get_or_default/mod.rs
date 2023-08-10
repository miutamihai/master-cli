use crate::config::get_or_default::create_default::create_default;
use crate::config::get_or_default::get::get;
use crate::config::get_or_default::parse::parse;
use crate::config::model::Config;
use crate::common::exit_with_error::exit_with_error;

mod create_default;
mod get;
mod parse;

pub fn get_or_default() -> Config {
    match get() {
        Ok(content) => match parse(content) {
            Ok(config) => config,
            Err(_) => {
                exit_with_error("Malformed config file")
            }
        },
        Err(_) => match create_default() {
            Ok(_) => get_or_default(),
            Err(_) => {
                exit_with_error("Failed to create default config")
            }
        },
    }
}
