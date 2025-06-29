//! Domain models and event types for the Kafka-in-Rust application.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Generic event envelope for all domain events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    /// Unique event identifier
    pub id: String,
    /// Tenant identifier for multi-tenancy
    pub tenant_id: String,
    /// Event type identifier
    pub event_type: String,
    /// Event payload as JSON
    pub payload: serde_json::Value,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event version for schema evolution
    pub version: String,
}

impl EventEnvelope {
    /// Create a new event envelope
    pub fn new(
        tenant_id: String,
        event_type: String,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id,
            event_type,
            payload,
            timestamp: Utc::now(),
            version: "1.0".to_string(),
        }
    }
    
    /// Get partition key for Kafka partitioning
    pub fn partition_key(&self) -> String {
        format!("{}|{}", self.tenant_id, self.event_type)
    }
} 