use log::error;
use std::process::exit;

pub trait FromString {
    fn from_string(input: String) -> Self;
}

pub enum ConfigNames {
    GitWorkDir,
    GitPersonalCredsName,
    GitPersonalCredsEmail,
    GitWorkCredsName,
    GitWorkCredsEmail,
}

impl ToString for ConfigNames {
    fn to_string(&self) -> String {
        match self {
            ConfigNames::GitWorkDir => String::from("git.work_dir"),
            ConfigNames::GitPersonalCredsName => String::from("git.personal_credentials.name"),
            ConfigNames::GitPersonalCredsEmail => String::from("git.personal_credentials.email"),
            ConfigNames::GitWorkCredsName => String::from("git.work_credentials.name"),
            ConfigNames::GitWorkCredsEmail => String::from("git.work_credentials.email"),
        }
    }
}

impl FromString for ConfigNames {
    fn from_string(input: String) -> Self {
        let static_string = input.as_str();

        match static_string {
            "git.work_dir" => ConfigNames::GitWorkDir,
            "git.personal_credentials.name" => ConfigNames::GitPersonalCredsName,
            "git.personal_credentials.email" => ConfigNames::GitPersonalCredsEmail,
            "git.work_credentials.name" => ConfigNames::GitWorkCredsName,
            "git.work_credentials.email" => ConfigNames::GitWorkCredsEmail,
            _ => {
                error!("Unknown config name: {}", static_string);
                exit(1)
            }
        }
    }
}
