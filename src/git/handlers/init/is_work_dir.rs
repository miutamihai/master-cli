use std::env;
use std::fs::canonicalize;

use crate::common::exit_with_errors::exit_with_errors;
use crate::config::model::Config;

pub fn is_work_dir(config: &Config) -> bool {
    match canonicalize(&config.git.work_dir) {
        Ok(path) => {
            let work_dir_path = path.as_path();

            if !work_dir_path.exists() {
                exit_with_errors("Current `git.work_dir` path doesn't exist")
            }

            match env::current_dir() {
                Ok(current_dir) => work_dir_path.eq(&current_dir),
                Err(_) => false,
            }
        }
        Err(_) => {
            exit_with_errors("Current `git.work_dir` path is invalid or not an absolute path")
        }
    }
}
