use serde_derive::{Deserialize, Serialize};

pub mod commands;
mod handlers;
pub mod match_command;

#[derive(Deserialize, Serialize, Clone)]
pub struct Profile {
    pub(crate) name: String,
}
