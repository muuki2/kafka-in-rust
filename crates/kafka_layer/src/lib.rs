//! Kafka layer providing abstraction over rdkafka for producers and consumers.

#![warn(missing_docs)]

pub mod producer;
pub mod consumer;

pub use producer::*;
pub use consumer::*; 