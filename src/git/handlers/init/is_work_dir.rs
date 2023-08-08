use crate::config::model::Config;
use std::env;

pub fn is_work_dir(config: &Config) -> bool {
    let work_dir = &config.git.work_dir;

    match env::current_dir() {
        Ok(current_dir) => current_dir
            .components()
            .any(|value| value.as_os_str().to_str().unwrap().eq(work_dir)),
        Err(_) => false,
    }
}
