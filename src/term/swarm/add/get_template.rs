use string_builder::Builder;

use crate::common::exit_with_errors::exit_with_errors;
use crate::embedded::settings::get::get;

pub fn get_template() -> String {
    let placeholder = get().config.default_value;
    let mut builder = Builder::default();

    builder.append(format!("# Replace {} with your own values\n", placeholder));
    builder.append("\n");
    builder.append("# The name to use for the swarm\n");
    builder.append(format!("name = \"{}\"\n", placeholder));
    builder.append("# The swarm's commands\n");
    builder.append(format!("commands = [\"{}\"]\n", placeholder));
    builder.append("# (Optional) Directory to run the swarm in\n");
    builder.append(format!("working_directory = \"{}\"\n", placeholder));
    builder.append("# (Optional) Prerequisite commands to run before the swarm\n");
    builder.append(format!("prerequisites = [\"{}\"]\n", placeholder));
    builder.append("# The swarm's type (window or tab)\n");
    builder.append("swarm_type = \"window\"\n");

    match builder.string() {
        Ok(template) => template,
        Err(error) => exit_with_errors(format!("Failed to get profile template: {}", error)),
    }
}
