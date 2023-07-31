pub trait MessageBuilder {
    fn build(prompt: &str, error: &str) -> Message;
}

pub struct Message {
    prompt: String,
    error: String
}
