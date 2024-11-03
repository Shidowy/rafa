use crate::broker::message::Message;
use crate::broker::topic::Topic;

pub struct Publisher {
    topic: Topic,
}

impl Publisher {
    pub fn new(topic: Topic) -> Self {
        Publisher { topic }
    }

    pub fn publish(&mut self, message: Message) {
        self.topic.publish(message);
    }
}