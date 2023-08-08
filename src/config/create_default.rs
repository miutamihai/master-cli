use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use log::{error, info};
use crate::config::config_path::config_path;
use crate::config::model::Config;

fn default() -> Config {
    Config {
        work_dir: String::from("work")
    }
}

pub fn create_default() -> std::io::Result<()> {
    let path = config_path();
    if let Some(parent) = &path.parent() {
        if fs::create_dir_all(parent).is_err() {
            error!("Failed to create config directory");

            exit(1)
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
