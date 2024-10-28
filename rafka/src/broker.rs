use std::collections::{HashMap, VecDeque};
use tokio::sync::{mpsc, Mutex};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub topic: String,
    pub content: String,
}

#[derive(Clone)]
pub struct Partition {
    id: usize,
    subscribers: Vec<mpsc::Sender<Message>>,
    log: VecDeque<Message>,  // In-memory message log for retention
    retention_limit: usize,   // Define the max size for log
}

impl Partition {
    fn new(id: usize, retention_limit: usize) -> Self {
        Partition {
            id,
            subscribers: Vec::new(),
            log: VecDeque::new(),
            retention_limit,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        if self.log.len() == self.retention_limit {
            self.log.pop_front(); // Remove the oldest message if limit is reached
        }
        self.log.push_back(message);
    }
}

#[derive(Clone)]
pub struct Broker {
    partitions: HashMap<String, Vec<Partition>>, // Map topics to partitions
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            partitions: HashMap::new(),
        }
    }

    pub async fn publish(&mut self, message: Message, partition_id: usize) {
        if let Some(partitions) = self.partitions.get_mut(&message.topic) {
            if let Some(partition) = partitions.get_mut(partition_id) {
                partition.add_message(message.clone()); // Add to log for retention
                for sub in &partition.subscribers {
                    let _ = sub.send(message.clone()).await;
                }
            }
        }
    }

    pub fn subscribe(&mut self, topic: String, partition_id: usize) -> mpsc::Receiver<Message> {
        let (tx, rx) = mpsc::channel(100);
        let partitions = self.partitions.entry(topic.clone()).or_insert_with(Vec::new);
    
        let current_len = partitions.len();
    
        if current_len <= partition_id {
            partitions.resize_with(partition_id + 1, || Partition::new(current_len, 100));
        }
    
        partitions[partition_id].subscribers.push(tx);
        rx
    }
}
