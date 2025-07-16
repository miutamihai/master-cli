use crate::common::exit_with_errors::exit_with_errors;
use crate::config::model::Config;
use crate::config::parse::parse;
use crate::config::read::read;

pub fn get() -> Config {
    match read() {
        Ok(content) => match parse(content) {
            Ok(config) => config,
            Err(error) => exit_with_errors(format!("Malformed config file: {}", error.message())),
        },
        Err(_) => exit_with_errors("Failed to read config file"),
    }
}
