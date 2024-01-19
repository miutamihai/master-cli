use anyhow::{anyhow, Result};

use first_add_config_version::Down as First;
use second_rename_version_field::Down as Second;
use third_add_swarm_prerequisites::Down as Third;

use super::model::Config as Current;
use super::write::write;

mod first_add_config_version;
mod fourth_add_swarm_type;
mod second_rename_version_field;
mod third_add_swarm_prerequisites;

pub trait Migration {
    type Up;

    fn to_up(&self) -> Self::Up;
    fn try_migrate(string: &String) -> Result<Current>;
    fn parse_string(toml_string: &String) -> Result<Self>
    where
        Self: Sized;
}

pub fn try_config_migration(config_string: String) -> Result<Current> {
    let maybe_migrated = vec![First::try_migrate, Second::try_migrate, Third::try_migrate]
        .iter()
        .find_map(|func| func(&config_string).ok());

    match maybe_migrated {
        Some(migrated) => {
            let clone = migrated.clone();

            write(clone, Some("Migrated config".to_string()));

            Ok(migrated)
        }
        None => Err(anyhow!("Failed to migrate")),
    }
}
