use std::collections::HashMap;

use crate::{profile::types::Profile, term::swarm::types::Swarm};
use serde_derive::{Deserialize, Serialize};

use super::Migration;
use crate::config::model::Config;

#[derive(Deserialize, Serialize, Clone)]
pub struct Down {
    pub version: String,
    pub current_profile: String,
    pub profiles: HashMap<String, Profile>,
    pub(crate) swarms: Vec<Swarm>,
}

impl Migration for Down {
    type Up = Config;

    fn to_up(previous: Self) -> Self::Up {
        Self::Up {
            config_version: previous.version,
            swarms: previous.swarms,
            profiles: previous.profiles,
            current_profile: previous.current_profile,
        }
    }

    fn parse_string(toml_string: &String) -> anyhow::Result<Self> {
        match toml::from_str::<Self>(toml_string) {
            Ok(result) => anyhow::Ok(result),
            Err(error) => Err(error.into()),
        }
    }

    fn try_migrate(string: &String) -> anyhow::Result<Config> {
        let parsed = Self::parse_string(string)?;

        Ok(Self::to_up(parsed))
    }
}
