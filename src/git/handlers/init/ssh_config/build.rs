use crate::common::exit_with_errors::exit_with_errors;
use string_builder::Builder;

pub fn build(key_path: &String) -> String {
    let mut builder = Builder::default();

    builder.append(format!("Host {}\n", key_path));
    builder.append(format!("\tUser git\n"));
    builder.append(format!("\tIdentityFile {}\n", key_path));
    builder.append(format!("\tIdentitiesOnly yes\n"));

    match builder.string() {
        Ok(config) => config,
        Err(error) => exit_with_errors(format!("Failed to build ssh_config config: {}", error)),
    }
}
