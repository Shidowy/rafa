use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use super::broker::Broker;

pub async fn consume(broker: Arc<Mutex<Broker>>, topic: &str, partition_id: usize) {
    let mut receiver = {
        let mut broker_locked = broker.lock().unwrap();
        broker_locked.subscribe(topic.to_string(), partition_id)
    };

    while let Some(message) = receiver.recv().await {
        println!("Received message on {}: {}", message.topic, message.content);

        // Acknowledge the message
        let mut broker_locked = broker.lock().unwrap();
        let acks = broker_locked.acknowledged_messages
            .entry(message.topic.clone())
            .or_insert_with(Vec::new); // Correctly initializes a new Vec if needed
        acks.push(message.clone()); // Store the acknowledged message

        // Log the acknowledgment
        log::info!("Acknowledged message on topic '{}': {}", message.topic, message.content);
    }
}
