use crate::config::model::Config;
use crate::config::names::ConfigNames;
use crate::config::validate::{validate, Rule};

pub fn validate_config(config: &Config) {
    validate(vec![
        Rule {
            name: ConfigNames::GitWorkDir,
            value: config.git.work_dir.clone(),
        },
        Rule {
            name: ConfigNames::GitPersonalCredsName,
            value: config.git.personal_credentials.name.clone(),
        },
        Rule {
            name: ConfigNames::GitPersonalCredsEmail,
            value: config.git.personal_credentials.email.clone(),
        },
        Rule {
            name: ConfigNames::GitPersonalCredsSshKey,
            value: config.git.personal_credentials.ssh_key.clone(),
        },
        Rule {
            name: ConfigNames::GitWorkCredsName,
            value: config.git.work_credentials.name.clone(),
        },
        Rule {
            name: ConfigNames::GitWorkCredsEmail,
            value: config.git.work_credentials.email.clone(),
        },
        Rule {
            name: ConfigNames::GitWorkCredsSshKey,
            value: config.git.work_credentials.ssh_key.clone(),
        },
    ])
}
