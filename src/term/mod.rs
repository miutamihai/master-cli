use serde_derive::{Deserialize, Serialize};

pub mod commands;
mod common;
pub mod match_command;
mod run;
pub mod swarm;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Terminal {
    Kitty,
    WezTerm,
}

impl From<Terminal> for String {
    fn from(val: Terminal) -> Self {
        match val {
            Terminal::Kitty => "kitty".to_string(),
            Terminal::WezTerm => "wezterm".to_string(),
        }
    }
}
