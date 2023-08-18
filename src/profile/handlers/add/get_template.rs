use string_builder::Builder;

use crate::common::exit_with_errors::exit_with_errors;
use crate::embedded::settings::get::get;

pub fn get_template() -> String {
    let placeholder = get().config.default_value;
    let mut builder = Builder::default();

    builder.append(format!("# Replace {} with your own values\n", placeholder));
    builder.append("\n");
    builder.append("# The name to use for the profile\n");
    builder.append(format!("name = \"{}\"\n", placeholder));

    match builder.string() {
        Ok(template) => template,
        Err(error) => exit_with_errors(format!("Failed to get profile template: {}", error)),
    }
}
