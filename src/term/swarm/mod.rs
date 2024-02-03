use self::commands::Commands;

mod add;
pub mod commands;
mod common;
mod run;
pub mod types;

use self::add::add;
use self::run::run;
use self::types::SwarmEnvironment;

use super::Terminal;

pub fn match_command(command: &Commands) {
    match command {
        Commands::Run { swarm_name } => run(swarm_name.clone()),
        Commands::Add => add(),
    }
}

impl SwarmEnvironment for Terminal {
    fn get_swarm_input(&self) -> types::SwarmInput {
        match self {
            Terminal::Kitty => types::SwarmInput {
                window_arguments: vec![
                    "--single-instance".to_string(),
                    "--hold".to_string(),
                    "sh".to_string(),
                    "-c".to_string(),
                ],
                tab_arguments: vec![
                    "--single-instance".to_string(),
                    "@".to_string(),
                    "launch".to_string(),
                    "--type".to_string(),
                    "tab".to_string(),
                    "sh".to_string(),
                    "-c".to_string(),
                ],
            },
            Terminal::WezTerm => todo!(),
        }
    }
}
