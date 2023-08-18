use edit::{edit_with_builder, Builder};

pub fn edit(template: String) -> std::io::Result<String> {
    let mut builder = Builder::default();
    builder = builder.suffix(".toml").to_owned();

    match edit_with_builder(template, &builder) {
        Ok(profile) => Ok(profile),
        Err(error) => Err(error),
    }
}
