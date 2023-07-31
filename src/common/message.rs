pub trait MessageBuilder {
    fn from_strings(prompt: &str, error: &str) -> Message;
}

pub struct Message {
    pub prompt: String,
    pub error: String
}

impl MessageBuilder for Message {
    fn from_strings(prompt: &str, error: &str) -> Message {
        Message {
            prompt: String::from(prompt),
            error: String::from(error)
        }
    }
}
