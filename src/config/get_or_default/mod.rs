use crate::common::exit_with_errors::exit_with_errors;
use crate::config::get_or_default::create_default::create_default;
use crate::config::get_or_default::get::get;
use crate::config::get_or_default::parse::parse;
use crate::config::model::Config;

mod create_default;
mod get;
mod parse;

pub fn get_or_default() -> Config {
    match get() {
        Ok(content) => match parse(content) {
            Ok(config) => config,
            Err(_) => exit_with_errors("Malformed config file"),
        },
        Err(_) => match create_default() {
            Ok(_) => get_or_default(),
            Err(_) => exit_with_errors("Failed to create default config"),
        },
    }
}
