use crate::embedded::settings::model::Settings;
use crate::embedded::Static;

pub fn get() -> Settings {
    // The following are safe to unwrap, since they're present at compile time
    let content = Static::get("settings.toml").unwrap();
    let string_content = String::from_utf8(content.data.to_vec()).unwrap();

    toml::from_str::<Settings>(&string_content).expect("Malformed settings file")
}
