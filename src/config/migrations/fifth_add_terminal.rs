use std::collections::HashMap;

use crate::{
    config::model::GitCredentials,
    profile::types::Profile,
    term::swarm::types::{Swarm, SwarmType},
};
use serde_derive::{Deserialize, Serialize};

use super::Migration;
use crate::config::model::Config;

#[derive(Deserialize, Serialize, Clone)]
pub struct DownSwarm {
    pub name: String,
    pub commands: Vec<String>,
    pub working_directory: Option<String>,
    pub prerequisites: Option<Vec<String>>,
    pub swarm_type: SwarmType,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Down {
    pub config_version: String,
    pub current_profile: String,
    pub profiles: HashMap<String, Profile>,
    pub(crate) swarms: Vec<DownSwarm>,
}

impl Migration for Down {
    type Up = Config;

    fn to_up(&self) -> Self::Up {
        // These are rewritten because we want compile
        // time errors if we forget to adjust these
        Self::Up {
            config_version: self.config_version.clone(),
            terminal: crate::term::Terminal::Kitty,
            swarms: self
                .swarms
                .iter()
                .map(|swarm| Swarm {
                    name: swarm.name.clone(),
                    working_directory: swarm.working_directory.clone(),
                    commands: swarm.commands.clone(),
                    prerequisites: swarm.prerequisites.clone(),
                    swarm_type: SwarmType::Window,
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

    fn try_migrate(string: &String) -> anyhow::Result<Config> {
        let parsed = Self::parse_string(string)?;

        Ok(parsed.to_up())
    }
}
