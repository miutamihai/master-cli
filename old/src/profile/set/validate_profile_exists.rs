use crate::common::exit_with_errors::exit_with_errors;
use crate::config::get::get;

pub fn validate_profile_exists(profile: String) {
    let config = get();

    if let None = config.profiles.get(profile.as_str()) {
        exit_with_errors(format!("Profile \"{}\" doesn't exist", profile))
    }
}
