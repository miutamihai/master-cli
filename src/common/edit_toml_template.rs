use crate::common::remove_toml_comments::remove_toml_comments;
use edit::{edit_with_builder, Builder};

pub fn edit_toml_template(template: String) -> std::io::Result<String> {
    let mut builder = Builder::default();
    builder = builder.suffix(".toml").to_owned();

    match edit_with_builder(template, &builder) {
        Ok(profile) => Ok(remove_toml_comments(profile)),
        Err(error) => Err(error),
    }
}
