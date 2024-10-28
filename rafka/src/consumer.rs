use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use super::broker::Broker;

pub async fn consume(broker: Arc<Mutex<Broker>>, topic: &str, partition_id: usize) {
    let mut receiver = broker.lock().unwrap().subscribe(topic.to_string(), partition_id); // Subscribe to a specific partition
    while let Some(message) = receiver.recv().await {
        println!("Received message on {} partition {}: {}", message.topic, partition_id, message.content);
    }
}
