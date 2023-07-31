use crate::common::run::run;
use crate::common::validate::validate;
use crate::git::handlers::restart::get_commands::get_commands;
use crate::git::handlers::restart::get_input::get_input;
use crate::git::handlers::restart::message::{DestinationMessage, OriginMessage};
use crate::common::message::Message;

mod build_input;
mod get_args;
mod get_commands;
mod get_input;
mod message;

pub fn restart() {
    let destination_messages = <Message as DestinationMessage>::build();
    let origin_messages = <Message as OriginMessage>::build();
    let destination = get_input(destination_messages.prompt);
    validate(destination != "", destination_messages.error);
    let origin = get_input(origin_messages.prompt);
    validate(origin != "", origin_messages.error);

    get_commands(destination, origin)
        .into_iter()
        .flat_map(run)
        .collect()
}
