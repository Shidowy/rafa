use crate::broker::topic::Topic;

pub struct Partition {
    pub id: u64,
    pub topic: Topic,
}

impl Partition {
    pub fn new(id: u64, topic: Topic) -> Self {
        Partition { id, topic }
    }
}