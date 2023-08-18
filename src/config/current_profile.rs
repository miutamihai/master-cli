use crate::common::exit_with_errors::exit_with_errors;
use crate::config::model::Config;
use crate::profile::Profile;

pub fn current_profile(config: &Config) -> Profile {
    let profile_name = config.current_profile.clone();

    match config.profiles.get(config.current_profile.as_str()) {
        None => exit_with_errors(format!("Profile {} doesn't exist", profile_name)),
        Some(profile) => profile.clone(),
    }
}
