use super::types::Swarm;
use crate::term::common::run_command;

pub fn run_swarm(swarm: Swarm) {
    if let Some(prerequisites) = swarm.prerequisites {
        prerequisites.iter().for_each(|command| {
            let _ = std::process::Command::new("sh")
                .args(vec!["-c", command])
                .output();
        });
    };

    swarm
        .commands
        .iter()
        .map(|command| command.clone())
        .for_each(|command| {
            run_command(command, swarm.working_directory.clone(), swarm.swarm_type)
        });
}
