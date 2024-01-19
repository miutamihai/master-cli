use std::collections::HashMap;

use crate::config::model::GitCredentials;
use crate::profile::types::Profile;
use serde_derive::{Deserialize, Serialize};

use super::second_rename_version_field::Down as Second;
use super::third_add_swarm_prerequisites::Down as Third;
use super::third_add_swarm_prerequisites::DownSwarm as Swarm;
use super::Migration;
use crate::embedded::settings::get::get;

#[derive(Deserialize, Serialize, Clone)]
pub struct Down {
    pub current_profile: String,
    pub profiles: HashMap<String, Profile>,
    pub(crate) swarms: Vec<Swarm>,
}

impl Migration for Down {
    type Up = Second;

    fn to_up(previous: Self) -> Self::Up {
        let version = get().config.config_version;

        // These are rewritten because we want compile
        // time errors if we forget to adjust these
        Self::Up {
            version,
            swarms: previous
                .swarms
                .iter()
                .map(|swarm| Swarm {
                    name: swarm.name.clone(),
                    working_directory: swarm.working_directory.clone(),
                    commands: swarm.commands.clone(),
                })
                .collect(),
            profiles: HashMap::from_iter(
                previous
                    .profiles
                    .iter()
                    .map(|(name, profile)| {
                        (
                            name.clone(),
                            Profile {
                                name: profile.name.clone(),
                                git_credentials: GitCredentials {
                                    name: profile.git_credentials.name.clone(),
                                    email: profile.git_credentials.email.clone(),
                                    ssh_key: profile.git_credentials.ssh_key.clone(),
                                },
                            },
                        )
                    })
                    .collect::<Vec<(String, Profile)>>(),
            ),
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

        Ok(Third::to_up(Second::to_up(Self::to_up(parsed))))
    }
}
