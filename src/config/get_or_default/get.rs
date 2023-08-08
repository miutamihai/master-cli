use std::fs;
use crate::config::config_path::config_path;

pub fn get() -> std::io::Result<String> {
    let path = config_path();

    fs::read_to_string(path)
}
