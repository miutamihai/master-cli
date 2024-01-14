use crate::common::exit_with_errors::exit_with_errors;
use crate::{config::current_profile::current_profile, profile::types::Profile};

use super::common::run_swarm;
use super::types::Swarm;

fn get_swarm(profile: Profile, swarm_name: String) -> Option<Swarm> {
    profile
        .swarms
        .iter()
        .find(|swarm| swarm.name == swarm_name)
        .cloned()
}

pub fn run(swarm_name: String) {
    let profile = current_profile();

    if let Some(swarm) = get_swarm(profile, swarm_name.clone()) {
        run_swarm(swarm)
    } else {
        exit_with_errors(format!("Swarm {} does not exist", swarm_name))
    }
}
