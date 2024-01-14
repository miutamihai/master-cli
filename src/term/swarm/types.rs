use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Swarm {
    pub name: String,
    pub commands: Vec<String>,
    pub working_directory: Option<String>,
}
