pub struct Message {
    pub id: u64,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn new(id: u64, payload: Vec<u8>) -> Self {
        Message { id, payload }
    }
}