use crate::config;
use crate::config::get::get;
use crate::config::names::{ConfigNames, FromString};
use log::warn;

pub fn inline(name: &String, value: &String) {
    warn!("Inline is disabled until further notice. No changes to the config will be made.");
    let config = get();
    let _config_name = ConfigNames::from_string(name.clone());

    // match config_name {
    //     ConfigNames::GitWorkDir => {
    //         config.git.work_dir = value.clone();
    //     }
    //     ConfigNames::GitPersonalCredsName => {
    //         config.git.personal_credentials.name = value.clone();
    //     }
    //     ConfigNames::GitPersonalCredsEmail => {
    //         config.git.personal_credentials.email = value.clone();
    //     }
    //     ConfigNames::GitPersonalCredsSshKey => {
    //         config.git.personal_credentials.ssh_key = value.clone();
    //     }
    //     ConfigNames::GitWorkCredsName => {
    //         config.git.work_credentials.name = value.clone();
    //     }
    //     ConfigNames::GitWorkCredsEmail => {
    //         config.git.work_credentials.email = value.clone();
    //     }
    //     ConfigNames::GitWorkCredsSshKey => {
    //         config.git.work_credentials.ssh_key = value.clone();
    //     }
    // };

    let success_message = format!("Changed config {} to {}", name, value);
    config::write::write(config, Some(success_message))
}
