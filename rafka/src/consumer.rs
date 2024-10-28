use std::sync::{Arc, Mutex}; 
use tokio::sync::mpsc;
use super::broker::Broker; 

// Update the function signature
pub async fn consume(broker: Arc<Mutex<Broker>>, topic: &str) {
    let mut receiver = broker.lock().unwrap().subscribe(topic.to_string()); // Lock the broker
    while let Some(message) = receiver.recv().await {
        println!("Received message on {}: {}", message.topic, message.content);
    }
}
