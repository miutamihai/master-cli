use crate::common::exit_with_errors::exit_with_errors;
use crate::config::config_path::config_path;
use crate::config::model::{Config, Git, GitCredentials};
use crate::embedded::settings::get::get;
use log::{error, info};
use std::fs;
use std::fs::File;
use std::io::Write;

fn default() -> Config {
    let value = String::from(get().config.default_value);

    Config {
        git: Git {
            work_dir: value.clone(),
            work_credentials: GitCredentials {
                name: value.clone(),
                email: value.clone(),
            },
            personal_credentials: GitCredentials {
                name: value.clone(),
                email: value.clone(),
            },
        },
    }
}

pub fn create_default() -> std::io::Result<()> {
    let path = config_path();
    if let Some(parent) = &path.parent() {
        if fs::create_dir_all(parent).is_err() {
            exit_with_errors("Failed to create config directory")
        }
    }

    match File::create(&path) {
        Ok(mut file) => {
            info!("Created file at {}", path.to_str().unwrap());
            let content = toml::to_string(&default()).unwrap();

            file.write_all(content.as_bytes())
        }
        Err(error) => {
            error!("Failed to create file because: {}", error.to_string());
            Ok(())
        }
    }
}
