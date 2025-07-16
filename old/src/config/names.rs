use crate::common::exit_with_errors::exit_with_errors;

pub trait FromString {
    fn from_string(input: String) -> Self;
}

pub enum ConfigNames {
    CurrentProfile,
}

impl ToString for ConfigNames {
    fn to_string(&self) -> String {
        match self {
            ConfigNames::CurrentProfile => String::from("current_profile"),
        }
    }
}

impl FromString for ConfigNames {
    fn from_string(input: String) -> Self {
        let static_string = input.as_str();

        match static_string {
            "current_profile" => ConfigNames::CurrentProfile,
            _ => exit_with_errors(format!("Unknown config name: {}", static_string)),
        }
    }
}
