use crate::common::run::Input;
use anyhow::*;

pub trait CommandInputFromString {
    fn from_string(string: String) -> Result<Input>;
}

impl CommandInputFromString for Input {
    fn from_string(string: String) -> Result<Input> {
        if let Some((command, arg_string)) = string.split_once(" ") {
            let args = {
                let ref this = arg_string.split(" ").collect::<Vec<&str>>();

                this.iter().map(|value| String::from(*value)).collect()
            };

            // let args = arg_string.split(" ").collect::<Vec<&str>>().as_string_vec();

            let input = Input {
                cmd: command.to_string(),
                args,
                on_done: None,
                on_error: None,
            };

            Ok(input)
        } else {
            Err(anyhow!("Invalid command"))
        }
    }
}
