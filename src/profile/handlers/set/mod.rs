mod validate_profile_exists;

use crate::config;
use crate::config::get::get;
use crate::config::model::Config;
use crate::profile::handlers::set::validate_profile_exists::validate_profile_exists;
use log::info;

fn set_current_profile(config: &mut Config, profile: String) {
    config.current_profile = profile;
}

fn log(profile: String) {
    info!("Profile `{}` was set as current", profile);
}

pub fn set(name: String) {
    validate_profile_exists(name.clone());
    let mut config = get();

    set_current_profile(&mut config, name.clone());

    config::write::write(config, None);
    log(name);
}
