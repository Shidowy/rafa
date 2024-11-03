use crate::broker::topic::Topic;

pub struct Subscriber {
    topic: Topic,
}

impl Subscriber {
    pub fn new(topic: Topic) -> Self {
        Subscriber { topic }
    }

    pub fn subscribe(&self) {
        // Logic for subscribing to messages
    }
}