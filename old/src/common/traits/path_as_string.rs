use std::fs::canonicalize;
use std::path::PathBuf;

use crate::common::exit_with_errors::exit_with_errors;

pub trait PathAsString {
    fn to_string(&self) -> String;
}

impl PathAsString for PathBuf {
    fn to_string(&self) -> String {
        match canonicalize(self) {
            Ok(canonical_path) => match canonical_path.to_str() {
                None => {
                    exit_with_errors("Failed to convert path to string");
                }
                Some(string_path) => string_path.to_string(),
            },
            Err(error) => {
                exit_with_errors(format!(
                    "Failed to canonicalize path: {}",
                    error.to_string()
                ));
            }
        }
    }
}
