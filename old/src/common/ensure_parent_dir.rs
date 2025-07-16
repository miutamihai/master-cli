use std::fs;
use std::path::PathBuf;

use crate::common::exit_with_errors::exit_with_errors;

pub fn ensure_parent_dir(path: &PathBuf, mut error_message: Option<String>) {
    let final_message = error_message.get_or_insert(String::from("Failed to create directory"));

    if let Some(parent) = &path.parent() {
        if fs::create_dir_all(parent).is_err() {
            exit_with_errors(format!("{}", final_message))
        }
    }
}
