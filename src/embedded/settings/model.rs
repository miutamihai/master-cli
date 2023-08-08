use serde_derive::{Deserialize};

#[derive(Deserialize)]
pub struct Config {
    pub(crate) default_value: String
}

#[derive(Deserialize)]
pub struct Settings {
    pub(crate) config: Config
}