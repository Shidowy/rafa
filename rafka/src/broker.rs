use std::collections::HashMap;
use tokio::sync::mpsc;
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

    // Publish a message to a specific partition within a topic
    pub async fn publish(&mut self, message: Message, partition_id: usize) {
        if let Some(partitions) = self.partitions.get_mut(&message.topic) {
            if let Some(partition) = partitions.get_mut(partition_id) {
                for sub in &partition.subscribers {
                    let _ = sub.send(message.clone()).await; // Send message to partition subscribers
                }
            } else {
                eprintln!("Partition ID {} does not exist for topic '{}'", partition_id, message.topic);
            }
        } else {
            eprintln!("Topic '{}' does not exist", message.topic);
        }
    }

    pub fn subscribe(&mut self, topic: String, partition_id: usize) -> mpsc::Receiver<Message> {
        let (tx, rx) = mpsc::channel(100);
    
        
        let partitions = self.partitions.entry(topic.clone()).or_insert_with(Vec::new);
    
       
        let current_len = partitions.len();
        if current_len <= partition_id {
            partitions.resize_with(partition_id + 1, || Partition {
                id: current_len, 
                subscribers: Vec::new(),
            });
        }
    
        
        partitions[partition_id].subscribers.push(tx);
        rx
    }    
}
