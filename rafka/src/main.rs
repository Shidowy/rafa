mod broker;
mod producer;
mod consumer;

use broker::Broker;
use std::sync::{Arc, Mutex}; 
use tokio;

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();
    let broker = Arc::new(Mutex::new(Broker::new())); 

    let topic = "news";
    let partition_id = 0; // Partition to use

    // Start a consumer for a specific topic and partition
    let broker_clone = Arc::clone(&broker); 
    tokio::spawn(async move {
        consumer::consume(broker_clone, topic, partition_id).await; 
    });

    // Produce some messages to the specified partition for tests
    {
        let mut broker_locked = broker.lock().unwrap(); // Lock the broker
        producer::produce(&mut broker_locked, topic, "Breaking news!", partition_id).await; // Pass mutable reference and partition_id
    }

    // Sleep for a while to allow messages to be processed
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}
