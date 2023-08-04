use crate::config::model::Config;
use std::fs;
use std::env;

pub fn get() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file_path = env::home_dir().ok_or("Home directory not found")?;
    file_path.push(".config/mm/config.toml");
    let absolute_path = fs::canonicalize(&file_path)?;
    let file_content = fs::read_to_string(absolute_path)?;
    let config: Config = toml::from_str(&file_content)?;

    Ok(config)
}