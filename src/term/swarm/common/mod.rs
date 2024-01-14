use super::types::Swarm;
use crate::term::common::run_term;

pub fn run_swarm(swarm: Swarm) {
    swarm
        .commands
        .iter()
        .map(|command| command.clone())
        .for_each(run_term);
}
