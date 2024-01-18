use std::collections::HashMap;

use crate::{profile::types::Profile, term::swarm::types::Swarm};
use serde_derive::{Deserialize, Serialize};

use super::second_rename_version_field::Down as Up;
use super::Migration;
use crate::embedded::settings::get::get;

#[derive(Deserialize, Serialize, Clone)]
pub struct Down {
    pub current_profile: String,
    pub profiles: HashMap<String, Profile>,
    pub(crate) swarms: Vec<Swarm>,
}

impl Migration for Down {
    type Up = Up;

    fn to_up(previous: Self) -> Self::Up {
        let version = get().config.config_version;

        Self::Up {
            version,
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

    fn try_migrate(string: &String) -> anyhow::Result<crate::config::model::Config> {
        let parsed = Self::parse_string(string)?;

        Ok(Up::to_up(Self::to_up(parsed)))
    }
}
