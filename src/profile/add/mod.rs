use super::types::Profile;
use crate::common::exit_with_errors::exit_with_errors;
use crate::config;
use crate::config::get::get;
use crate::config::model::Config;
use crate::profile::add::edit::edit;
use crate::profile::add::get_template::get_template;
use crate::profile::add::parse::parse;
use log::info;

mod edit;
mod get_template;
mod parse;

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

    match edit(template) {
        Ok(editted) => {
            let profile = parse(editted);
            let mut config = get();

            set_current_profile(&mut config, profile.clone());
            add_profile(&mut config, profile.clone());

            config::write::write(config, None);
            log(profile);
        }
        Err(error) => exit_with_errors(error.to_string()),
    }
}
