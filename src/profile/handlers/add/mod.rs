use crate::common::exit_with_errors::exit_with_errors;
use crate::config;
use crate::config::model::Config;
use crate::profile::handlers::add::edit::edit;
use crate::profile::handlers::add::get_template::get_template;
use crate::profile::handlers::add::parse::parse;
use crate::profile::Profile;
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

pub fn add(config: Config) {
    let template = get_template();

    match edit(template) {
        Ok(editted) => {
            let profile = parse(editted);
            let mut copy = config;

            set_current_profile(&mut copy, profile.clone());
            add_profile(&mut copy, profile.clone());

            config::write::write(copy, None);
            log(profile);
        }
        Err(error) => exit_with_errors(error.to_string()),
    }
}
