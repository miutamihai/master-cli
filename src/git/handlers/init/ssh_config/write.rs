use crate::common::ensure_parent_dir::ensure_parent_dir;
use crate::common::exit_with_errors::exit_with_errors;
use crate::config::config_path::config_folder;
use log::info;
use sha256::digest;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn get_file_name() -> std::io::Result<String> {
    let project_path = std::env::current_dir()?;

    Ok(digest(project_path.to_str().unwrap()))
}

fn build_path() -> std::io::Result<PathBuf> {
    let path = config_folder();
    let file_name = get_file_name()?;

    Ok(path.join("ssh_configs").join(file_name))
}

pub fn write(content: String) -> std::io::Result<PathBuf> {
    let path = build_path()?;
    ensure_parent_dir(
        &path,
        Some(String::from("Failed to create ssh_configs directory")),
    );

    match File::create(&path) {
        Ok(mut file) => {
            info!("Created ssh config at {}", path.to_str().unwrap());

            file.write_all(content.as_bytes())?;
            Ok(PathBuf::from(path))
        }
        Err(error) => exit_with_errors(format!(
            "Failed to create ssh config because: {}",
            error.to_string()
        )),
    }
}
