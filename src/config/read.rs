use crate::config::config_path::config_path;
use std::fs;

pub fn read() -> std::io::Result<String> {
    let path = config_path();

    fs::read_to_string(path)
}
