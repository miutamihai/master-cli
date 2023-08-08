use crate::config::names::ConfigNames;
use crate::embedded::settings::get::get;
use log::error;
use std::process::exit;

pub struct Rule {
    pub(crate) name: ConfigNames,
    pub(crate) value: String,
}

pub fn validate(values: Vec<Rule>) {
    let default_value = get().config.default_value;

    values.iter().for_each(|rule| {
        if rule.value == default_value {
            error!("`{}` value not set!", rule.name.to_string());
            error!(
                "Please run: `mm config --name {} --value <your_value>`",
                rule.name.to_string()
            );
            exit(1)
        }
    })
}
