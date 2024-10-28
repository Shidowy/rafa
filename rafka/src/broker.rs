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
    messages: Vec<Message>, // Store messages in the partition
    retention_limit: usize,  // Set a retention limit
}

impl Partition {
    // Update this method to include the retention limit
    pub fn new(id: usize, retention_limit: usize) -> Self {
        Partition {
            id,
            subscribers: Vec::new(),
            messages: Vec::new(),  // Initialize empty messages
            retention_limit,
        }
    }

    // Add a message and enforce retention
    pub fn add_message(&mut self, message: Message) {
        if self.messages.len() >= self.retention_limit {
            self.messages.remove(0); 
        }
        self.messages.push(message);
    }
}


#[derive(Clone)]
pub struct Broker {
    partitions: HashMap<String, Vec<Partition>>,
    pub acknowledged_messages: HashMap<String, Vec<Message>>, 
    acknowledged_counts: HashMap<String, usize>,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            partitions: HashMap::new(),
            acknowledged_messages: HashMap::new(),
            acknowledged_counts: HashMap::new(),
        }
    }

    pub async fn publish(&mut self, message: Message, partition_id: usize) {
    
        let partitions = self.partitions.entry(message.topic.clone()).or_insert_with(Vec::new);
    
        // Create the necessary partition if it doesn't exist
        while partitions.len() <= partition_id {
            partitions.push(Partition::new(partitions.len(), 100)); // Adjust the new method to your needs
            log::info!("Created new partition {} for topic '{}'", partition_id, message.topic);
        }
    
        // Publish the message to the specified partition
        if let Some(partition) = partitions.get_mut(partition_id) {
            for sub in &partition.subscribers {
                let _ = sub.send(message.clone()).await; // Send message to partition subscribers
            }
            log::info!("Message sent to topic '{}', partition {}", message.topic, partition_id);
        } else {
            log::error!("Partition {} not found for topic '{}'", partition_id, message.topic);
        }
    }
    

    pub fn subscribe(&mut self, topic: String, partition_id: usize) -> mpsc::Receiver<Message> {
        let (tx, rx) = mpsc::channel(100);
    
        // Clone the topic for logging and to avoid moving it
        let topic_clone = topic.clone();
    
        // Ensure the partition exists
        let partition = self.partitions.entry(topic_clone.clone()).or_insert_with(Vec::new);
        
        while partition.len() <= partition_id {
            partition.push(Partition::new(partition.len(), 100)); // Create a new partition if it doesn't exist
        }
    
        // Log the subscription
        log::info!("Subscriber added to topic '{}', partition {}", topic_clone, partition_id);
    
        
        let partition_acks = self.acknowledged_messages
            .entry(topic_clone.clone())
            .or_insert_with(Vec::new); 
    
        
        partition_acks.entry(partition_id).or_insert_with(Vec::new);
    
        // Add the subscriber to the partition
        partition[partition_id].subscribers.push(tx);
        rx
    }
    
    // Optional: A method for logging status
    pub fn log_status(&self) {
        for (topic, partitions) in &self.partitions {
            log::info!("Topic: '{}', Partitions: {}", topic, partitions.len());
            for partition in partitions {
                log::info!("  Partition {} has {} subscribers.", partition.id, partition.subscribers.len());
            }
        }
    }    
}