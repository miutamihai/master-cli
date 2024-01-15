use crate::common::exit_with_errors::exit_with_errors;
use crate::config::get::get;
use crate::config::model::Config;

use super::common::run_swarm;
use super::types::Swarm;

fn get_swarm(config: Config, swarm_name: String) -> Option<Swarm> {
    config
        .swarms
        .iter()
        .find(|swarm| swarm.name == swarm_name)
        .cloned()
}

pub fn run(swarm_name: String) {
    let config = get();

    if let Some(swarm) = get_swarm(config, swarm_name.clone()) {
        run_swarm(swarm)
    } else {
        exit_with_errors(format!("Swarm {} does not exist", swarm_name))
    }
}
