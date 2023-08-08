use std::process::exit;
use log::error;
use crate::config::get_or_default::create_default::create_default;
use crate::config::get_or_default::get::get;
use crate::config::get_or_default::parse::parse;
use crate::config::model::Config;

mod get;
mod parse;
mod create_default;

pub fn get_or_default() -> Config {
    match get() {
        Ok(content) => {
            match parse(content) {
                Ok(config) => config,
                Err(_) => {
                    error!("Malformed config file");

                    exit(1)
                }
            }
        }
        Err(_) => {
            match create_default() {
                Ok(_) => {
                    get_or_default()
                }
                Err(_) => {
                    error!("Failed to create default config");

                    exit(1)
                }
            }
        }
    }
}
