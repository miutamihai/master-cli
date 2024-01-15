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
    builder.append("# The git credentials to use with this profile\n");
    builder.append("[git_credentials]\n");
    builder.append("# The name to sign your commits with\n");
    builder.append(format!("name = \"{}\"\n", placeholder));
    builder.append("# The email to sign your commits with\n");
    builder.append(format!("email = \"{}\"\n", placeholder));
    builder.append(
        "# The absolute path to the private ssh key to use when interacting with the remote\n",
    );
    builder.append(format!("ssh_key = \"{}\"\n", placeholder));

    match builder.string() {
        Ok(template) => template,
        Err(error) => exit_with_errors(format!("Failed to get profile template: {}", error)),
    }
}
