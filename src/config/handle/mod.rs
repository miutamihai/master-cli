use crate::config::model::Config;
use crate::config::names::{ConfigNames, FromString};

pub fn handle(name: &String, value: &String, config: Config) {
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
        ConfigNames::GitWorkCredsName => {
            copy.git.work_credentials.name = value.clone();
        }
        ConfigNames::GitWorkCredsEmail => {
            copy.git.work_credentials.email = value.clone();
        }
    };
}
