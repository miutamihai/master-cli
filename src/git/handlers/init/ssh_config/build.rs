use std::fs::canonicalize;

use string_builder::Builder;

use crate::common::exit_with_errors::exit_with_errors;

pub fn build(key_path: &String) -> String {
    let mut builder = Builder::default();
    let canonical_path = match canonicalize(key_path) {
        Ok(path) => path,
        Err(_) => exit_with_errors(format!("Key path {} is invalid", key_path)),
    };

    builder.append(format!("Host {}\n", key_path));
    builder.append(format!("\tUser git\n"));
    builder.append(format!(
        "\tIdentityFile {}\n",
        canonical_path.to_str().unwrap()
    ));
    builder.append(format!("\tIdentitiesOnly yes\n"));

    match builder.string() {
        Ok(config) => config,
        Err(error) => exit_with_errors(format!("Failed to build ssh_config config: {}", error)),
    }
}
