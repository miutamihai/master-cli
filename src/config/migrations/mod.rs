use anyhow::{anyhow, Result};

use first_add_config_version::Down as First;
use second_rename_version_field::Down as Second;

use super::model::Config as Current;
use super::write::write;

mod first_add_config_version;
mod second_rename_version_field;

pub trait Migration {
    type Up;

    fn to_up(previous: Self) -> Self::Up;
    fn try_migrate(string: &String) -> Result<Current>;
    fn parse_string(toml_string: &String) -> Result<Self>
    where
        Self: Sized;
}

pub fn try_config_migration(config_string: String) -> Result<Current> {
    let migrations = vec![
        First::try_migrate(&config_string),
        Second::try_migrate(&config_string),
    ];

    let maybe_migrated = migrations.into_iter().find_map(|result| result.ok());

    match maybe_migrated {
        Some(migrated) => {
            let clone = migrated.clone();

            write(clone, Some("Migrated config".to_string()));

            Ok(migrated)
        }
        None => Err(anyhow!("Failed to migrate")),
    }
}
