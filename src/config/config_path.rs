use std::path::PathBuf;

pub fn config_path() -> PathBuf {
    dirs::config_local_dir().unwrap().join("mm/config.toml")
}
