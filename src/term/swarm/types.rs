use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SwarmType {
    Window,
    Tab,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Swarm {
    pub name: String,
    pub commands: Vec<String>,
    pub working_directory: Option<String>,
    pub prerequisites: Option<Vec<String>>,
    pub swarm_type: SwarmType,
}
