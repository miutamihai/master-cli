use crate::config::config_path::config_path;
use crate::config::model::Config;
use crate::config::names::{ConfigNames, FromString};
use fs::write;
use log::{error, info};
use std::fs;
use toml::to_string;

pub fn inline(name: &String, value: &String, config: Config) {
    let mut copy = config;
    let config_name = ConfigNames::from_string(name.clone());

    match config_name {
        ConfigNames::GitWorkDir => {
            copy.git.work_dir = value.clone();
        }
        ConfigNames::GitPersonalCredsName => {
            copy.git.personal_credentials.name = value.clone();
        }
        ConfigNames::GitPersonalCredsEmail => {
            copy.git.personal_credentials.email = value.clone();
        }
        ConfigNames::GitPersonalCredsSshKey => {
            copy.git.personal_credentials.ssh_key = value.clone();
        }
        ConfigNames::GitWorkCredsName => {
            copy.git.work_credentials.name = value.clone();
        }
        ConfigNames::GitWorkCredsEmail => {
            copy.git.work_credentials.email = value.clone();
        }
        ConfigNames::GitWorkCredsSshKey => {
            copy.git.work_credentials.ssh_key = value.clone();
        }
    };

    let content = to_string(&copy).unwrap();

    match write(config_path(), content) {
        Ok(_) => {
            info!("Changed config {} to {}", name, value)
        }
        Err(_) => {
            error!("Failed to write config change!")
        }
    }
}
