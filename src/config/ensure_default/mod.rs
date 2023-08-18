use crate::common::exit_with_errors::exit_with_errors;
use crate::config::ensure_default::create_default::create_default;
use crate::config::model::Config;
use crate::config::parse::parse;
use crate::config::read::read;

mod create_default;

pub fn ensure_default() -> Config {
    match read() {
        Ok(content) => match parse(content) {
            Ok(config) => config,
            Err(_) => exit_with_errors("Malformed config file"),
        },
        Err(_) => match create_default() {
            Ok(_) => ensure_default(),
            Err(_) => exit_with_errors("Failed to create default config"),
        },
    }
}
