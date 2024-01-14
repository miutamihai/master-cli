use super::types::Profile;
use crate::common::edit_toml_template::edit_toml_template;
use crate::common::exit_with_errors::exit_with_errors;
use crate::common::parse_toml_string::parse_toml_string;
use crate::config;
use crate::config::get::get;
use crate::config::model::Config;
use crate::profile::add::get_template::get_template;
use log::info;

mod get_template;

fn add_profile(config: &mut Config, profile: Profile) {
    let profile_name = profile.name.clone();

    config.profiles.insert(String::from(profile_name), profile);
}

fn set_current_profile(config: &mut Config, profile: Profile) {
    config.current_profile = profile.name;
}

fn log(profile: Profile) {
    info!("Newly added profile `{}` was set as current", profile.name);
}

pub fn add() {
    let template = get_template();

    match edit_toml_template(template) {
        Ok(editted) => {
            let profile: Profile = parse_toml_string(editted, "Failed to parse new profile");
            let mut config = get();

            set_current_profile(&mut config, profile.clone());
            add_profile(&mut config, profile.clone());

            config::write::write(config, None);
            log(profile);
        }
        Err(error) => exit_with_errors(error.to_string()),
    }
}
