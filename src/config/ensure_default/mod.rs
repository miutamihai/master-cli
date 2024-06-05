use super::ensure_default::create_default::create_default;
use super::migrations::try_config_migration;
use super::model::Config;
use super::parse::parse;
use super::read::read;
use crate::common::exit_with_errors::exit_with_errors;

mod create_default;

pub fn ensure_default() -> Config {
    match read() {
        Ok(content) => match parse(content.clone()) {
            Ok(config) => config,
            Err(error) => {
                dbg!(error);

                match try_config_migration(content) {
                    Ok(config) => config,
                    Err(error) => exit_with_errors(format!(
                        "Migration failed, malformed config file: {}",
                        error
                    )),
                }
            }
        },
        Err(_) => match create_default() {
            Ok(_) => ensure_default(),
            Err(_) => exit_with_errors("Failed to create default config"),
        },
    }
}
