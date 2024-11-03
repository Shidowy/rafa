use crate::broker::message::Message;

pub struct InMemoryStorage {
    pub messages: Vec<Message>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        InMemoryStorage { messages: Vec::new() }
    }

    pub fn store_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}