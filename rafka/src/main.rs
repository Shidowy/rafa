mod broker;
mod producer;
mod consumer;

use broker::Broker;
use std::sync::{Arc, Mutex}; 
use tokio;

#[tokio::main]
async fn main() {
    let broker = Arc::new(Mutex::new(Broker::new())); 

    let topic = "news";

    // Start a consumer for a specific topic
    let broker_clone = Arc::clone(&broker); 
    tokio::spawn(async move {
        consumer::consume(broker_clone, topic).await; 
    });

    // Produce some messages
    {
        let mut broker_locked = broker.lock().unwrap(); // Lock the broker
        producer::produce(&mut broker_locked, topic, "Breaking news!").await; // Pass mutable reference
    }

    // Sleep for a while to allow messages to be processed
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}
