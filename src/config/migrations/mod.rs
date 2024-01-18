use anyhow::{anyhow, Result};

use first_add_config_version::Previous as First;

use super::model::Config as Current;
use super::write::write;

mod first_add_config_version;

pub trait Migration {
    type Up;

    fn migrate(previous: Self) -> Self::Up;
    fn parse_string(toml_string: String) -> Result<Self>
    where
        Self: Sized;
}

pub fn try_config_migration(config_string: String) -> Result<Current> {
    if let Ok(matched_config) = First::parse_string(config_string) {
        let migrated = First::migrate(matched_config);
        let clone = migrated.clone();

        write(clone, Some("Migrated config".to_string()));

        Ok(migrated)
    } else {
        Err(anyhow!("Failed to migrate"))
    }
}
