//! Kafka layer providing abstraction over rdkafka for producers and consumers.

#![warn(missing_docs)]

pub mod consumer;
pub mod producer;

pub use consumer::*;
pub use producer::*;
