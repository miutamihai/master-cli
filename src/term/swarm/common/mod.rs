use super::types::Swarm;
use crate::term::common::run_term;

pub fn run_swarm(swarm: Swarm) {
    if let Some(prerequisites) = swarm.prerequisites {
        prerequisites
            .iter()
            .for_each(|command| match command.split_once(" ") {
                Some((program, args_string)) => {
                    let args: Vec<&str> = args_string.split_whitespace().collect();

                    let _ = std::process::Command::new(program).args(args).output();
                }
                None => (),
            });
    }

    swarm
        .commands
        .iter()
        .map(|command| command.clone())
        .for_each(|command| run_term(command, swarm.working_directory.clone()));
}
