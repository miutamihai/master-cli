use super::{common::run_command, swarm::types::SwarmType};

pub fn run(command: String) {
    run_command(command, None, SwarmType::Window);
}
