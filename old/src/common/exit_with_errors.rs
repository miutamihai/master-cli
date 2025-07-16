use std::process::exit;

use log::error;

use crate::common::traits::as_string_vec::AsStringVec;

pub fn exit_with_errors<T: AsStringVec>(input: T) -> ! {
    let messages = input.as_string_vec();

    for message in messages {
        error!("{}", message);
    }

    exit(1)
}
