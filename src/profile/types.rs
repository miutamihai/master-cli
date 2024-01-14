use crate::config::model::GitCredentials;
use crate::term::swarm::types::Swarm;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Profile {
    pub(crate) name: String,
    pub(crate) git_credentials: GitCredentials,
    pub(crate) swarms: Vec<Swarm>,
}
