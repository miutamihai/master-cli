use rust_embed::RustEmbed;

pub mod settings;

#[derive(RustEmbed)]
#[folder = "static"]
struct Static;
