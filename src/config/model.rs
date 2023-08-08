use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub(crate) work_dir: String
}
