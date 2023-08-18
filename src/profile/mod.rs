use serde_derive::{Deserialize, Serialize};

pub mod commands;
pub mod match_command;

#[derive(Deserialize, Serialize)]
pub struct Profile {
    pub(crate) name: String,
}
