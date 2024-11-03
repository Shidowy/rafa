use super::broker::{Broker, Message};

pub async fn produce(broker: &mut Broker, topic: &str, content: &str, partition_id: usize) {
    let message = Message {
        topic: topic.to_string(),
        content: content.to_string(),
    };
    broker.publish(message, partition_id).await;
}
