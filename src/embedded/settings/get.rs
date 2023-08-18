use crate::embedded::settings::model::Settings;
use crate::embedded::Static;
use lazy_static::lazy_static;
use std::ops::Deref;
use std::sync::Mutex;

lazy_static! {
    static ref SETTINGS: Mutex<Settings> = {
        let content = Static::get("settings.toml").unwrap();
        let string_content = String::from_utf8(content.data.to_vec()).unwrap();
        let settings =
            toml::from_str::<Settings>(&string_content).expect("Malformed settings file");
        Mutex::new(settings)
    };
}

pub fn get() -> Settings {
    let locked_settings = SETTINGS.lock().unwrap();
    locked_settings.deref().clone()
}
