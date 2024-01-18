use std::collections::HashMap;

use crate::{profile::types::Profile, term::swarm::types::Swarm};
use serde_derive::{Deserialize, Serialize};

use super::Migration;
use crate::config::model::Config;
use crate::embedded::settings::get::get;

#[derive(Deserialize, Serialize, Clone)]
pub struct Previous {
    pub current_profile: String,
    pub profiles: HashMap<String, Profile>,
    pub(crate) swarms: Vec<Swarm>,
}

impl Migration for Previous {
    type Up = Config;

    fn migrate(previous: Self) -> Config {
        let version = get().config.config_version;

        Config {
            version,
            swarms: previous.swarms,
            profiles: previous.profiles,
            current_profile: previous.current_profile,
        }
    }

    fn parse_string(toml_string: String) -> anyhow::Result<Self> {
        match toml::from_str::<Self>(&toml_string) {
            Ok(result) => anyhow::Ok(result),
            Err(error) => Err(error.into()),
        }
    }
}
