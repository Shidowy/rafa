use config::{Config, ConfigBuilder, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub log: LogConfig,
    pub broker: BrokerConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct BrokerConfig {
    pub replication_factor: u8,
    pub default_topic_partitions: u8,
}

#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    pub r#type: String,
}

impl Settings {
    pub fn new(environment: &str) -> Result<Self, config::ConfigError> {
        // Initialize the ConfigBuilder
        let builder = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", environment)).required(false))
            .add_source(Environment::with_prefix("APP").separator("_"));

        // Build the configuration
        builder.build()?.try_deserialize()
    }
}