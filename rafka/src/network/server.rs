pub struct Server;

impl Server {
    pub fn new() -> Self {
        Server
    }

    pub async fn start(&self) {
        println!("Server started...");
        // Server logic here
    }
}