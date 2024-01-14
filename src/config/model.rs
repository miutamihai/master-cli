use crate::profile::types::Profile;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone)]
pub struct GitCredentials {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) ssh_key: String,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub current_profile: String,
    pub profiles: HashMap<String, Profile>,
}
