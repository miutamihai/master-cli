use crate::config::get::get;
use crate::config::model::Config;
use crate::config::names::ConfigNames;
use crate::config::validate::{validate, Rule};

fn build_rules(config: Config) -> Vec<Rule> {
    vec![Rule {
        name: ConfigNames::CurrentProfile,
        value: config.current_profile,
    }]
}

fn get_error_message() -> String {
    "No current profile set! Please set one by running `mm profile set` or add a new one by running `mm profile add`.".to_string()
}

pub fn validate_profile_set() {
    let config = get();

    validate(build_rules(config), Some(get_error_message()));
}
