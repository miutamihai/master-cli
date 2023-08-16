use crate::common::exit_with_errors::exit_with_errors;

pub trait FromString {
    fn from_string(input: String) -> Self;
}

pub enum ConfigNames {
    GitWorkDir,
    GitPersonalCredsName,
    GitPersonalCredsEmail,
    GitPersonalCredsSshKey,
    GitWorkCredsName,
    GitWorkCredsEmail,
    GitWorkCredsSshKey,
}

impl ToString for ConfigNames {
    fn to_string(&self) -> String {
        match self {
            ConfigNames::GitWorkDir => String::from("git.work_dir"),
            ConfigNames::GitPersonalCredsName => String::from("git.personal_credentials.name"),
            ConfigNames::GitPersonalCredsEmail => String::from("git.personal_credentials.email"),
            ConfigNames::GitPersonalCredsSshKey => String::from("git.personal_credentials.ssh_key"),
            ConfigNames::GitWorkCredsName => String::from("git.work_credentials.name"),
            ConfigNames::GitWorkCredsEmail => String::from("git.work_credentials.email"),
            ConfigNames::GitWorkCredsSshKey => String::from("git.work_credentials.ssk_key"),
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
            "git.personal_credentials.ssh_key" => ConfigNames::GitPersonalCredsSshKey,
            "git.work_credentials.name" => ConfigNames::GitWorkCredsName,
            "git.work_credentials.email" => ConfigNames::GitWorkCredsEmail,
            "git.work_credentials.ssh_key" => ConfigNames::GitWorkCredsSshKey,
            _ => exit_with_errors(format!("Unknown config name: {}", static_string)),
        }
    }
}
