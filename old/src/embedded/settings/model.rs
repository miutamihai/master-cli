use serde_derive::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub(crate) default_value: String,
    pub(crate) config_version: String,
}

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub(crate) config: Config,
}
