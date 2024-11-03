use thiserror::Error;

#[derive(Error, Debug)]
pub enum RafkaError {
    #[error("Network error")]
    NetworkError,
    #[error("Storage error")]
    StorageError,
    // Add more errors as needed
}