use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GitCredentials {
    pub(crate) name: String,
    pub(crate) email: String
}

#[derive(Deserialize, Serialize)]
pub struct Git {
    pub(crate) work_dir: String,
    pub(crate) personal_credentials: GitCredentials,
    pub(crate) work_credentials: GitCredentials
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub git: Git
}
