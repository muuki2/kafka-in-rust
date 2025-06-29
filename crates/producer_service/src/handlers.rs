//! Event handlers for the producer service.

use anyhow::Result;
use commons::EventEnvelope;
use kafka_layer::KafkaProducer;

/// Producer service for handling event generation
pub struct ProducerService {
    /// Kafka producer instance
    producer: KafkaProducer,
}

impl ProducerService {
    /// Create a new producer service
    pub fn new() -> Result<Self> {
        Ok(Self {
            producer: KafkaProducer::new()?,
        })
    }
    
    /// Send an event
    pub async fn send_event(&self, event: EventEnvelope) -> Result<()> {
        self.producer.send_event(&event).await
    }
} 