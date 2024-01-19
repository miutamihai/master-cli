use std::collections::HashMap;

use crate::{config::model::GitCredentials, profile::types::Profile};
use serde_derive::{Deserialize, Serialize};

use super::third_add_swarm_prerequisites::Down as Third;
use super::third_add_swarm_prerequisites::DownSwarm as Swarm;
use super::Migration;

#[derive(Deserialize, Serialize, Clone)]
pub struct Down {
    pub version: String,
    pub current_profile: String,
    pub profiles: HashMap<String, Profile>,
    pub(crate) swarms: Vec<Swarm>,
}

impl Migration for Down {
    type Up = Third;

    fn to_up(&self) -> Self::Up {
        // These are rewritten because we want compile
        // time errors if we forget to adjust these
        Self::Up {
            config_version: self.version.clone(),
            swarms: self
                .swarms
                .iter()
                .map(|swarm| Swarm {
                    name: swarm.name.clone(),
                    working_directory: swarm.working_directory.clone(),
                    commands: swarm.commands.clone(),
                })
                .collect(),
            profiles: HashMap::from_iter(
                self.profiles
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
            current_profile: self.current_profile.clone(),
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

        Ok(parsed.to_up().to_up().to_up())
    }
}
