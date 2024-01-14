use super::exit_with_errors::exit_with_errors;
use serde::Deserialize;

pub fn parse_toml_string<T: for<'de> Deserialize<'de>>(content: String, error_prefix: &str) -> T {
    match toml::from_str::<T>(content.as_str()) {
        Ok(parsed) => parsed,
        Err(error) => exit_with_errors(format!("{}: {}", error_prefix, error.to_string())),
    }
}
