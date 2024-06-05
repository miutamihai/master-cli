use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use log::info;

use crate::common::ensure_parent_dir::ensure_parent_dir;
use crate::common::exit_with_errors::exit_with_errors;
use crate::config::config_path::config_path;
use crate::config::model::Config;
use crate::embedded::settings::get::get;

fn default() -> Config {
    let value = get().config.default_value;
    let version = get().config.config_version;

    Config {
        config_version: version,
        current_profile: value,
        profiles: HashMap::new(),
        swarms: vec![],
        terminal: crate::term::Terminal::Kitty,
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
        Err(error) => exit_with_errors(format!("Failed to create config file because: {}", error)),
    }
}
