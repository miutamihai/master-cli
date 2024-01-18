use crate::{profile::types::Profile, term::swarm::types::Swarm};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone)]
pub struct GitCredentials {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) ssh_key: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub version: String,
    pub current_profile: String,
    pub profiles: HashMap<String, Profile>,
    pub(crate) swarms: Vec<Swarm>,
}
