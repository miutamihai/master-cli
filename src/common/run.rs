use std::process::{Child, Command};

type SuccessHandler = fn() -> ();

type ErrorHandler = fn(&String) -> ();

#[derive(Clone, Debug)]
pub struct Input {
    pub(crate) cmd: String,
    pub(crate) args: Vec<String>,
    pub(crate) on_done: Option<SuccessHandler>,
    pub(crate) on_error: Option<ErrorHandler>,
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        let cmd_equals = self.cmd == other.cmd;
        let args_equal = self.args == other.args;

        cmd_equals && args_equal
    }
}

fn build_command(cmd: String, args: Vec<String>) -> Command {
    let mut command = Command::new(cmd);
    command.args(args);

    command
}

fn wait_for_child(mut child: Child, input: Input) -> Result<(), String> {
    match child.wait() {
        Ok(_) => {
            if let Some(on_done) = input.on_done {
                on_done()
            }

            Ok(())
        }
        Err(error) => { Err(error.to_string()) }
    }
}

pub fn run(input: Input) -> Result<(), String> {
    match build_command(input.clone().cmd, input.clone().args).spawn() {
        Ok(child) => {
            wait_for_child(child, input)
        }
        Err(error) => {
            if let Some(on_error) = input.on_error {
                on_error(&error.to_string());
            }

            Err(error.to_string())
        }
    }
}