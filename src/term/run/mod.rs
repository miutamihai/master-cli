use super::{
    common::run_command,
    swarm::types::{Swarm, SwarmType},
};

pub fn run(command: String) {
    let swarm = Swarm {
        swarm_type: SwarmType::Window,
        name: "mock".to_string(),
        working_directory: None,
        commands: vec![],
        prerequisites: None,
    };

    run_command(command, swarm);
}
