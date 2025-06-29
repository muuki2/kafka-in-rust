//! Configuration management for the Kafka-in-Rust application.

use serde::{Deserialize, Serialize};

/// Main application configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Kafka configuration
    pub kafka: KafkaConfig,
    /// Producer configuration  
    pub producer: ProducerConfig,
    /// Consumer configuration
    pub consumer: ConsumerConfig,
    /// Observability configuration
    pub observability: ObservabilityConfig,
}

/// Kafka broker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    /// Kafka bootstrap servers
    pub bootstrap_servers: String,
    /// Security protocol
    pub security_protocol: String,
}

/// Producer-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProducerConfig {
    /// Events topic name
    pub events_topic: String,
    /// Retry topic name
    pub retry_topic: String,
    /// Dead letter queue topic name
    pub dlq_topic: String,
    /// Delivery timeout in milliseconds
    pub delivery_timeout_ms: u32,
}

/// Consumer-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerConfig {
    /// Consumer group ID
    pub group_id: String,
    /// Batch size for processing
    pub batch_size: usize,
    /// Enable auto offset commit
    pub auto_commit: bool,
}

/// Observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    /// Log level
    pub log_level: String,
    /// Enable JSON logging
    pub json_logs: bool,
    /// Metrics server address
    pub metrics_addr: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            kafka: KafkaConfig {
                bootstrap_servers: "localhost:9092".to_string(),
                security_protocol: "PLAINTEXT".to_string(),
            },
            producer: ProducerConfig {
                events_topic: "events".to_string(),
                retry_topic: "retry".to_string(),
                dlq_topic: "dlq".to_string(),
                delivery_timeout_ms: 120_000,
            },
            consumer: ConsumerConfig {
                group_id: "kafka-rust-consumer".to_string(),
                batch_size: 100,
                auto_commit: false,
            },
            observability: ObservabilityConfig {
                log_level: "info".to_string(),
                json_logs: false,
                metrics_addr: "127.0.0.1:9090".to_string(),
            },
        }
    }
}
