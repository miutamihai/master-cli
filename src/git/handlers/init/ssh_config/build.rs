use std::path::PathBuf;

use string_builder::Builder;

use crate::common::exit_with_errors::exit_with_errors;
use crate::common::traits::path_as_string::PathAsString;

pub fn build(key_path: &String) -> String {
    let mut builder = Builder::default();
    let canonical_path = PathBuf::from(key_path).to_string();

    builder.append(format!("Host {}\n", key_path));
    builder.append(format!("\tUser git\n"));
    builder.append(format!("\tIdentityFile {}\n", canonical_path));
    builder.append(format!("\tIdentitiesOnly yes\n"));

    match builder.string() {
        Ok(config) => config,
        Err(error) => exit_with_errors(format!("Failed to build ssh_config config: {}", error)),
    }
}
