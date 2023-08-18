use crate::profile::Profile;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone)]
pub struct GitCredentials {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) ssh_key: String,
}

#[derive(Deserialize, Serialize)]
pub struct Git {
    pub(crate) work_dir: String,
    pub(crate) personal_credentials: GitCredentials,
    pub(crate) work_credentials: GitCredentials,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub git: Git,
    pub current_profile: String,
    pub profiles: HashMap<String, Profile>,
}
