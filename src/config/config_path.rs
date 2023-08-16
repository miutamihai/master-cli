use std::path::PathBuf;

pub fn config_folder() -> PathBuf {
    dirs::config_local_dir().unwrap().join("mm")
}

pub fn config_path() -> PathBuf {
    config_folder().join("config.toml")
}
