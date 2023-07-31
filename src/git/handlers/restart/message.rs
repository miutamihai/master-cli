use crate::common::message::{Message, MessageBuilder};

pub trait OriginMessage {
    fn build() -> Message;
}

pub trait DestinationMessage {
    fn build() -> Message;
}

impl OriginMessage for Message {
    fn build() -> Message {
        const PROMPT: &str = "What's your destination branch? (branch you want to work with)";
        const ERROR: &str = "Destination must not be empty!";
        Message::from_strings(PROMPT, ERROR)
    }
}

impl DestinationMessage for Message {
    fn build() -> Message {
        const PROMPT: &str = "What's your origin branch? (branch you want to rebase from)";
        const ERROR: &str = "Origin must not be empty!";
        Message::from_strings(PROMPT, ERROR)
    }
}
