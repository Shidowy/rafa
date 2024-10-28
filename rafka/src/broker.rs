use std::collections::HashMap;
use tokio::sync::{mpsc, Mutex};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub topic: String,
    pub content: String,
}

#[derive(Clone)]
pub struct Broker {
    subscribers: HashMap<String, Vec<mpsc::Sender<Message>>>,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            subscribers: HashMap::new(),
        }
    }

    // Publish a message to a topic
    pub async fn publish(&mut self, message: Message) {
        if let Some(subs) = self.subscribers.get(&message.topic) {
            for sub in subs {
                let _ = sub.send(message.clone()).await; // Send message to subscribers
            }
        }
    }

    // Subscribe to a topic
    pub fn subscribe(&mut self, topic: String) -> mpsc::Receiver<Message> {
        let (tx, rx) = mpsc::channel(100);
        self.subscribers.entry(topic).or_default().push(tx);
        rx
    }
}
