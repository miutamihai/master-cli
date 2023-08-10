use crate::common::exit_with_errors::exit_with_errors;
use crate::config::names::ConfigNames;
use crate::embedded::settings::get::get;

pub struct Rule {
    pub(crate) name: ConfigNames,
    pub(crate) value: String,
}

pub fn validate(values: Vec<Rule>) {
    let default_value = get().config.default_value;

    values.iter().for_each(|rule| {
        if rule.value == default_value {
            exit_with_errors(vec![
                format!("`{}` value not set!", rule.name.to_string()),
                format!(
                    "Please run: `mm config --name {} --value <your_value>`",
                    rule.name.to_string()
                ),
            ])
        }
    })
}
