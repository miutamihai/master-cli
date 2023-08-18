use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use log::info;

use crate::common::ensure_parent_dir::ensure_parent_dir;
use crate::common::exit_with_errors::exit_with_errors;
use crate::config::config_path::config_path;
use crate::config::model::{Config, Git, GitCredentials};
use crate::embedded::settings::get::get;
use crate::profile::Profile;

fn default() -> Config {
    let value = String::from(get().config.default_value);

    Config {
        current_profile: Profile {
            name: value.clone(),
            git_credentials: GitCredentials {
                name: value.clone(),
                email: value.clone(),
                ssh_key: value.clone(),
            },
        },
        profiles: HashMap::new(),
        git: Git {
            work_dir: value.clone(),
            work_credentials: GitCredentials {
                name: value.clone(),
                email: value.clone(),
                ssh_key: value.clone(),
            },
            personal_credentials: GitCredentials {
                name: value.clone(),
                email: value.clone(),
                ssh_key: value.clone(),
            },
        },
    }
}

pub fn create_default() -> std::io::Result<()> {
    let path = config_path();
    ensure_parent_dir(
        &path,
        Some(String::from("Failed to create config directory")),
    );

    match File::create(&path) {
        Ok(mut file) => {
            info!("Created config file at {}", path.to_str().unwrap());
            let content = toml::to_string(&default()).unwrap();

            file.write_all(content.as_bytes())
        }
        Err(error) => exit_with_errors(format!(
            "Failed to create config file because: {}",
            error.to_string()
        )),
    }
}
