use crate::common::exit_with_errors::exit_with_errors;
use crate::profile::Profile;

pub fn parse(content: String) -> Profile {
    match toml::from_str::<Profile>(&content) {
        Ok(profile) => profile,
        Err(error) => exit_with_errors(format!(
            "Failed to parse new profile: {}",
            error.to_string()
        )),
    }
}
