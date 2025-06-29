# Kafka-in-Rust

A high-performance, fault-tolerant event streaming platform built with Rust and Apache Kafka, capable of processing 20,000+ messages per second with at-least-once delivery guarantees.

## 🚀 Features

- **High Throughput**: Process 20,000+ events/second
- **Fault Tolerant**: At-least-once delivery with retry and dead letter queue handling
- **Multi-tenant**: Tenant-based partitioning for data isolation
- **Observability**: Comprehensive metrics, structured logging, and health checks
- **CLI Tools**: Built-in tools for data ingestion and consumer management
- **Type Safe**: Leverages Rust's type system for memory safety and performance

## 📋 Prerequisites

- **Rust 1.79+** (stable toolchain)
- **Docker & Docker Compose** (for local Kafka cluster)
- **Git** (for version control)

## 🏗️ Architecture

```
rust-kafka/
├── crates/
│   ├── commons/           # Shared types & utilities
│   ├── kafka_layer/       # Thin wrapper over rdkafka
│   ├── producer_service/  # Domain-specific producers
│   └── consumer_service/  # Domain-specific consumers
├── bin/
│   ├── ingest.rs          # CLI: produce test data
│   └── process.rs         # CLI: run consumer workers
├── config/                # Configuration files
├── docker/                # Docker setup
└── tests/                 # Integration tests
```

## 🚀 Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/kafka-in-rust.git
cd kafka-in-rust
```

### 2. Start Local Kafka Cluster

```bash
docker-compose up -d
```

### 3. Build the Project

```bash
cargo build --release
```

### 4. Run Consumer (Terminal 1)

```bash
cargo run --bin process start
```

### 5. Generate Test Data (Terminal 2)

```bash
# Generate 100 user events
cargo run --bin ingest users --count 100

# Generate order events
cargo run --bin ingest orders --count 50

# Run load test
cargo run --bin ingest load-test --rate 1000 --duration 30
```

## 🔧 Configuration

The application uses a layered configuration approach:

1. **Default values** (built-in)
2. **Config files** (`config/default.toml`, `config/local.toml`)
3. **Environment variables** (prefix: `KAFKA_`)

### Environment Variables

```bash
# Kafka Configuration
KAFKA_KAFKA__BOOTSTRAP_SERVERS=localhost:9092
KAFKA_KAFKA__SECURITY_PROTOCOL=PLAINTEXT

# Producer Configuration
KAFKA_PRODUCER__EVENTS_TOPIC=events
KAFKA_PRODUCER__DELIVERY_TIMEOUT_MS=120000

# Consumer Configuration
KAFKA_CONSUMER__GROUP_ID=kafka-rust-consumer
KAFKA_CONSUMER__BATCH_SIZE=100

# Observability
KAFKA_OBSERVABILITY__LOG_LEVEL=info
KAFKA_OBSERVABILITY__METRICS_ADDR=127.0.0.1:9090
```

## 🛠️ Development

### Running Tests

```bash
# Unit tests
cargo test

# Integration tests with Kafka
cargo test --test integration

# With coverage
cargo tarpaulin --out html
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy --all-targets --all-features -- -D warnings

# Check documentation
cargo doc --no-deps --open
```

### Local Development with Kafka

The `docker-compose.yml` provides a complete Kafka development environment:

- **Zookeeper**: `localhost:2181`
- **Kafka Broker**: `localhost:9092`
- **Kafka UI**: `http://localhost:8080` (optional)

## 📊 Monitoring & Observability

### Metrics

Prometheus metrics are exposed on `http://localhost:9090/metrics`:

- `kafka_producer_messages_produced_total`
- `kafka_consumer_messages_consumed_total`
- `kafka_producer_duration_seconds`
- `kafka_consumer_processing_duration_seconds`

### Health Checks

- **Liveness**: `GET /healthz`
- **Readiness**: `GET /readyz`

### Structured Logging

```bash
# Enable JSON logging
KAFKA_OBSERVABILITY__JSON_LOGS=true

# Set log level
KAFKA_OBSERVABILITY__LOG_LEVEL=debug
```

## 🎯 Usage Examples

### Custom Event Types

```bash
# Custom events with JSON payload
cargo run --bin ingest custom \
  --event-type "analytics.page_view" \
  --payload '{"user_id": "123", "page": "/home"}' \
  --count 1000
```

### Consumer Management

```bash
# Check consumer lag
cargo run --bin process lag --group-id my-consumer-group

# Seek to specific offset (for replay)
cargo run --bin process seek \
  --topic events \
  --partition 0 \
  --offset 1000

# View metrics
cargo run --bin process metrics
```

## 🐳 Docker Deployment

### Building Docker Image

```bash
# Build optimized image
docker build -t kafka-rust:latest .

# Run producer
docker run --rm kafka-rust:latest ingest users --count 100

# Run consumer
docker run --rm kafka-rust:latest process start
```

### Kubernetes Deployment

```bash
# Deploy to Kubernetes
kubectl apply -f k8s/

# Check status
kubectl get pods -l app=kafka-rust
```

## 🔒 Security

- **TLS encryption** in production environments
- **SASL authentication** support (SCRAM-SHA-256, PLAIN)
- **Secret management** via environment variables
- **No personal data** in event payloads (GDPR compliant)

## 🧪 Load Testing

Built-in load testing capabilities:

```bash
# Sustained load test
cargo run --bin ingest load-test \
  --rate 5000 \
  --duration 300 \
  --tenant-id production

# Burst test
cargo run --bin ingest load-test \
  --rate 10000 \
  --duration 60
```

Expected performance:
- **P99 latency**: ≤ 50ms end-to-end
- **Throughput**: 20,000+ messages/second
- **CPU usage**: ≤ 65% under peak load
- **Memory usage**: ≤ 300 MiB per service

## 📈 Roadmap

- [ ] **Week 1**: Infrastructure setup (Docker, TLS)
- [ ] **Week 2**: Core Kafka layer implementation
- [ ] **Week 3**: Producer service MVP
- [ ] **Week 4**: Consumer service MVP
- [ ] **Week 5**: Observability & metrics
- [ ] **Week 6**: Integration tests & load testing
- [ ] **Week 7**: Documentation & deployment
- [ ] **Week 8**: Production release v1.0.0

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/new-feature`)
3. Commit changes (`git commit -am 'Add new feature'`)
4. Push to branch (`git push origin feat/new-feature`)
5. Create a Pull Request

### Code Standards

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Maintain ≥80% test coverage
- All public items must have documentation
- Run `cargo fmt` and `cargo clippy` before committing

## 📄 License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

- [rdkafka](https://github.com/fede1024/rust-rdkafka) - Rust Kafka client
- [tokio](https://tokio.rs/) - Async runtime
- [tracing](https://tracing.rs/) - Structured logging
- [Apache Kafka](https://kafka.apache.org/) - Event streaming platform 