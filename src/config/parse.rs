use crate::config::model::Config;
use toml::de::Error;

pub fn parse(content: String) -> Result<Config, Error> {
    toml::from_str::<Config>(&content)
}
