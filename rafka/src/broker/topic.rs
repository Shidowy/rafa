use crate::broker::message::Message;

pub struct Topic {
    pub name: String,
    pub messages: Vec<Message>,
}

impl Topic {
    pub fn new(name: String) -> Self {
        Topic {
            name,
            messages: Vec::new(),
        }
    }

    pub fn publish(&mut self, message: Message) {
        self.messages.push(message);
    }
}