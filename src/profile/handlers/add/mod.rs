use crate::common::exit_with_errors::exit_with_errors;
use crate::config;
use crate::config::model::Config;
use crate::profile::handlers::add::edit::edit;
use crate::profile::handlers::add::get_template::get_template;
use crate::profile::handlers::add::parse::parse;

mod edit;
mod get_template;
mod parse;

pub fn add(config: Config) {
    let template = get_template();

    match edit(template) {
        Ok(editted) => {
            let profile = parse(editted);
            let profile_name = &profile.name;
            let mut copy = config;

            copy.profiles.insert(String::from(profile_name), profile);

            config::write::write(copy, None);
        }
        Err(error) => exit_with_errors(error.to_string()),
    }
}
