use crate::common::exit_with_errors::exit_with_errors;
use crate::config::config_path::config_folder;
use crate::git::handlers::init::ssh_config::model::Type;
use log::info;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

// TODO: Remove this and use current directory as name
fn build_path(key_type: Type) -> PathBuf {
    let mut path = config_folder().into_os_string();

    match key_type {
        Type::Personal => {
            path.push("/personal_ssh_config");
            path.into()
        }
        Type::Work => {
            path.push("/work_ssh_config");
            path.into()
        }
    }
}

pub fn write(content: String, key_type: Type) -> std::io::Result<PathBuf> {
    let path = build_path(key_type);

    match File::create(&path) {
        Ok(mut file) => {
            info!("Created ssh config at {}", path.to_str().unwrap());

            file.write_all(content.as_bytes())?;
            Ok(path)
        }
        Err(error) => exit_with_errors(format!(
            "Failed to create ssh config because: {}",
            error.to_string()
        )),
    }
}
