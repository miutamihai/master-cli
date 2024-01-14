mod get_template;

use get_template::get_template;
use log::info;

use crate::{
    common::{
        edit_toml_template::edit_toml_template, exit_with_errors::exit_with_errors,
        parse_toml_string::parse_toml_string,
    },
    config::{self, current_profile::current_profile, get::get, model::Config},
};

use super::types::Swarm;

fn add_swarm(config: &mut Config, swarm: Swarm) {
    let mut profile = current_profile();
    profile.swarms.push(swarm);

    config
        .profiles
        .insert(config.current_profile.clone(), profile);
}

fn log(swarm: Swarm) {
    info!("Swarm {} added", swarm.name);
}

pub fn add() {
    let template = get_template();

    match edit_toml_template(template) {
        Ok(editted) => {
            let swarm: Swarm = parse_toml_string(editted, "Failed to parse new swarm");
            let mut config = get();

            add_swarm(&mut config, swarm.clone());
            config::write::write(config, None);
            log(swarm);
        }
        Err(error) => exit_with_errors(error.to_string()),
    }
}
