use std::path::PathBuf;

pub fn config_folder() -> PathBuf {
    let dir = dirs::config_local_dir().unwrap();

    if cfg!(debug_assertions) {
        dir.join("mm").join("dev")
    } else {
        dir.join("mm")
    }
}

pub fn config_path() -> PathBuf {
    config_folder().join("config.toml")
}
