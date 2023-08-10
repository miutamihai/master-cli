use std::process::exit;
use log::error;

pub trait AsStaticStrings {
    fn as_static_strings(&self) -> Vec<&'static str>;
}

impl AsStaticStrings for &'static str {
    fn as_static_strings(&self) -> Vec<&'static str> {
        vec![self]
    }
}

impl AsStaticStrings for Vec<&'static str> {
    fn as_static_strings(&self) -> Vec<&'static str> {
        self.clone()
    }
}

pub fn exit_with_error<T: AsStaticStrings>(input: T) -> ! {
    let messages = input.as_static_strings();

    for message in messages {
        error!("{}", message);
    }

    exit(1)
}