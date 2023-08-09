use crate::config::model::Config;
use std::env;
use std::path::Path;
use std::process::exit;
use log::error;

pub fn is_work_dir(config: &Config) -> bool {
    let work_dir_path = Path::new(&config.git.work_dir);

    if !work_dir_path.exists() {
        error!("Current `git.work_dir` path doesn't exist");

        exit(1)
    }

    match env::current_dir() {
        Ok(current_dir) => {
            work_dir_path.eq(&current_dir)
        },
        Err(_) => false,
    }
}
