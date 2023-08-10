use crate::common::exit_with_errors::exit_with_errors;
use crate::config::model::Config;
use std::env;
use std::path::Path;

pub fn is_work_dir(config: &Config) -> bool {
    let work_dir_path = Path::new(&config.git.work_dir);

    if !work_dir_path.exists() {
        exit_with_errors("Current `git.work_dir` path doesn't exist")
    }

    match env::current_dir() {
        Ok(current_dir) => work_dir_path.eq(&current_dir),
        Err(_) => false,
    }
}
