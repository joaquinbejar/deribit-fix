# Benchmarking Guide

Benchmarking is essential for ensuring the `deribit-fix` crate meets performance requirements and for detecting performance regressions. This guide covers performance testing strategies, tools, and best practices.

## Overview

Benchmarking helps us:
- Measure performance characteristics
- Detect performance regressions
- Optimize critical code paths
- Validate performance requirements
- Compare different implementations

## Performance Requirements

The `deribit-fix` crate has the following performance targets:

- **Latency**: < 1ms for order placement
- **Throughput**: 10,000+ messages per second
- **Memory Usage**: < 10MB under normal load
- **CPU Usage**: < 5% under normal load
- **Connection Recovery**: < 100ms for reconnection

## Benchmarking Tools

### Criterion.rs

Criterion is the primary benchmarking framework for Rust. It provides statistical analysis and regression detection.

```toml
# Cargo.toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "message_parsing"
harness = false

[[bench]]
name = "order_placement"
harness = false

[[bench]]
name = "connection_establishment"
harness = false
```

### Basic Benchmark Structure

```rust
// benches/message_parsing.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deribit_fix::*;

fn benchmark_message_parsing(c: &mut Criterion) {
    let fix_string = "8=FIX.4.4|35=D|49=CLIENT|56=DERIBIT|34=1|52=20231201-10:00:00.000|";
    
    c.bench_function("parse_fix_message", |b| {
        b.iter(|| {
            let _message = FixMessage::from_str(black_box(fix_string)).unwrap();
        });
    });
}

fn benchmark_message_serialization(c: &mut Criterion) {
    let message = FixMessage::new()
        .with_msg_type("D")
        .with_sender_comp_id("CLIENT")
        .with_target_comp_id("DERIBIT")
        .with_msg_seq_num(1)
        .with_sending_time(chrono::Utc::now());
    
    c.bench_function("serialize_fix_message", |b| {
        b.iter(|| {
            let _serialized = black_box(&message).to_string();
        });
    });
}

criterion_group!(benches, benchmark_message_parsing, benchmark_message_serialization);
criterion_main!(benches);
```

## Core Performance Benchmarks

### Message Handling Benchmarks

```rust
// benches/message_handling.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deribit_fix::*;

fn benchmark_message_creation(c: &mut Criterion) {
    c.bench_function("create_fix_message", |b| {
        b.iter(|| {
            let _message = FixMessage::new()
                .with_msg_type("D")
                .with_sender_comp_id("CLIENT")
                .with_target_comp_id("DERIBIT")
                .with_msg_seq_num(1)
                .with_sending_time(chrono::Utc::now());
        });
    });
}

fn benchmark_message_validation(c: &mut Criterion) {
    let message = FixMessage::new()
        .with_msg_type("D")
        .with_sender_comp_id("CLIENT")
        .with_target_comp_id("DERIBIT")
        .with_msg_seq_num(1)
        .with_sending_time(chrono::Utc::now());
    
    c.bench_function("validate_fix_message", |b| {
        b.iter(|| {
            let _result = black_box(&message).validate();
        });
    });
}

fn benchmark_message_parsing_batch(c: &mut Criterion) {
    let fix_strings = vec![
        "8=FIX.4.4|35=D|49=CLIENT|56=DERIBIT|34=1|52=20231201-10:00:00.000|",
        "8=FIX.4.4|35=8|49=DERIBIT|56=CLIENT|34=1|52=20231201-10:00:00.000|",
        "8=FIX.4.4|35=0|49=DERIBIT|56=CLIENT|34=1|52=20231201-10:00:00.000|",
    ];
    
    c.bench_function("parse_fix_message_batch", |b| {
        b.iter(|| {
            for fix_string in &fix_strings {
                let _message = FixMessage::from_str(black_box(fix_string)).unwrap();
            }
        });
    });
}

criterion_group!(benches, 
    benchmark_message_creation, 
    benchmark_message_validation, 
    benchmark_message_parsing_batch
);
criterion_main!(benches);
```

### Order Management Benchmarks

```rust
// benches/order_management.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deribit_fix::*;

fn benchmark_order_creation(c: &mut Criterion) {
    c.bench_function("create_order", |b| {
        b.iter(|| {
            let _order = Order::new()
                .with_symbol("BTC-PERPETUAL")
                .with_side(OrderSide::Buy)
                .with_order_type(OrderType::Limit)
                .with_quantity(0.1)
                .with_price(50000.0)
                .with_time_in_force(TimeInForce::Day);
        });
    });
}

fn benchmark_order_validation(c: &mut Criterion) {
    let order = Order::new()
        .with_symbol("BTC-PERPETUAL")
        .with_side(OrderSide::Buy)
        .with_order_type(OrderType::Limit)
        .with_quantity(0.1)
        .with_price(50000.0)
        .with_time_in_force(TimeInForce::Day);
    
    c.bench_function("validate_order", |b| {
        b.iter(|| {
            let _result = black_box(&order).validate();
        });
    });
}

fn benchmark_order_to_fix_message(c: &mut Criterion) {
    let order = Order::new()
        .with_symbol("BTC-PERPETUAL")
        .with_side(OrderSide::Buy)
        .with_order_type(OrderType::Limit)
        .with_quantity(0.1)
        .with_price(50000.0)
        .with_time_in_force(TimeInForce::Day);
    
    c.bench_function("order_to_fix_message", |b| {
        b.iter(|| {
            let _message = black_box(&order).to_fix_message().unwrap();
        });
    });
}

criterion_group!(benches, 
    benchmark_order_creation, 
    benchmark_order_validation, 
    benchmark_order_to_fix_message
);
criterion_main!(benches);
```

### Connection and Session Benchmarks

```rust
// benches/connection_session.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deribit_fix::*;
use tokio::runtime::Runtime;

fn benchmark_session_creation(c: &mut Criterion) {
    c.bench_function("create_session_manager", |b| {
        b.iter(|| {
            let _session = SessionManager::new()
                .with_sender_comp_id("CLIENT")
                .with_target_comp_id("DERIBIT")
                .with_heartbeat_interval(30);
        });
    });
}

fn benchmark_sequence_management(c: &mut Criterion) {
    let mut session = SessionManager::new();
    
    c.bench_function("sequence_number_increment", |b| {
        b.iter(|| {
            let _seq_num = black_box(&mut session).next_outbound_seq_num();
        });
    });
}

fn benchmark_heartbeat_generation(c: &mut Criterion) {
    let session = SessionManager::new()
        .with_sender_comp_id("CLIENT")
        .with_target_comp_id("DERIBIT");
    
    c.bench_function("generate_heartbeat", |b| {
        b.iter(|| {
            let _heartbeat = black_box(&session).generate_heartbeat();
        });
    });
}

criterion_group!(benches, 
    benchmark_session_creation, 
    benchmark_sequence_management, 
    benchmark_heartbeat_generation
);
criterion_main!(benches);
```

## Async Performance Benchmarks

### Async Operation Benchmarks

```rust
// benches/async_operations.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deribit_fix::*;
use tokio::runtime::Runtime;

fn benchmark_async_order_placement(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("async_order_placement", |b| {
        b.to_async(&rt).iter(|| async {
            let client = DeribitFixClient::new(Config::default());
            let order = Order::new()
                .with_symbol("BTC-PERPETUAL")
                .with_side(OrderSide::Buy)
                .with_order_type(OrderType::Limit)
                .with_quantity(0.1)
                .with_price(50000.0);
            
            let _result = client.place_order(black_box(order)).await;
        });
    });
}

fn benchmark_async_market_data_subscription(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("async_market_data_subscription", |b| {
        b.to_async(&rt).iter(|| async {
            let client = DeribitFixClient::new(Config::default());
            let subscription = MarketDataSubscription::new()
                .with_symbol("BTC-PERPETUAL")
                .with_depth(10);
            
            let _stream = client.subscribe_market_data(black_box(subscription)).await;
        });
    });
}

criterion_group!(benches, 
    benchmark_async_order_placement, 
    benchmark_async_market_data_subscription
);
criterion_main!(benches);
```

## Load Testing Benchmarks

### High-Frequency Trading Benchmarks

```rust
// benches/load_testing.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deribit_fix::*;
use tokio::runtime::Runtime;

fn benchmark_high_frequency_orders(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("high_frequency_order_placement", |b| {
        b.to_async(&rt).iter(|| async {
            let client = DeribitFixClient::new(Config::default());
            let order_count = 1000;
            
            for i in 0..order_count {
                let order = Order::new()
                    .with_symbol("BTC-PERPETUAL")
                    .with_side(OrderSide::Buy)
                    .with_order_type(OrderType::Limit)
                    .with_quantity(0.01)
                    .with_price(50000.0 + i as f64);
                
                let _result = client.place_order(black_box(order)).await;
            }
        });
    });
}

fn benchmark_concurrent_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("concurrent_order_operations", |b| {
        b.to_async(&rt).iter(|| async {
            let client = DeribitFixClient::new(Config::default());
            let operation_count = 100;
            
            let handles: Vec<_> = (0..operation_count)
                .map(|i| {
                    let client = client.clone();
                    tokio::spawn(async move {
                        let order = Order::new()
                            .with_symbol("BTC-PERPETUAL")
                            .with_side(OrderSide::Buy)
                            .with_order_type(OrderType::Limit)
                            .with_quantity(0.01)
                            .with_price(50000.0 + i as f64);
                        
                        client.place_order(black_box(order)).await
                    })
                })
                .collect();
            
            let _results = futures::future::join_all(handles).await;
        });
    });
}

criterion_group!(benches, 
    benchmark_high_frequency_orders, 
    benchmark_concurrent_operations
);
criterion_main!(benches);
```

## Memory and Resource Benchmarks

### Memory Usage Benchmarks

```rust
// benches/memory_usage.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deribit_fix::*;
use std::alloc::{alloc, dealloc, Layout};

fn benchmark_memory_allocation(c: &mut Criterion) {
    c.bench_function("memory_allocation", |b| {
        b.iter(|| {
            let layout = Layout::new::<FixMessage>();
            unsafe {
                let ptr = alloc(layout);
                dealloc(ptr, layout);
            }
        });
    });
}

fn benchmark_message_memory_usage(c: &mut Criterion) {
    c.bench_function("message_memory_usage", |b| {
        b.iter(|| {
            let messages: Vec<FixMessage> = (0..100)
                .map(|i| {
                    FixMessage::new()
                        .with_msg_type("D")
                        .with_sender_comp_id("CLIENT")
                        .with_target_comp_id("DERIBIT")
                        .with_msg_seq_num(i)
                        .with_sending_time(chrono::Utc::now())
                })
                .collect();
            
            black_box(messages);
        });
    });
}

criterion_group!(benches, 
    benchmark_memory_allocation, 
    benchmark_message_memory_usage
);
criterion_main!(benches);
```

## Configuration and Customization

### Criterion Configuration

```rust
// benches/bench_config.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(100)           // Number of samples
        .confidence_level(0.95)     // Confidence level
        .significance_level(0.05)   // Significance level
        .warm_up_time(std::time::Duration::from_millis(100))
        .measurement_time(std::time::Duration::from_secs(1))
}

fn benchmark_with_config(c: &mut Criterion) {
    let mut group = c.benchmark_group("custom_group");
    group.measurement_time(std::time::Duration::from_secs(2));
    group.sample_size(200);
    
    group.bench_function("custom_benchmark", |b| {
        b.iter(|| {
            // Benchmark code here
        });
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_with_config);
criterion_main!(benches);
```

## Running Benchmarks

### Basic Commands

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench message_parsing

# Run benchmarks with output
cargo bench -- --verbose

# Run benchmarks with specific features
cargo bench --features performance_testing

# Run benchmarks with custom parameters
cargo bench -- --sample-size 200 --measurement-time 2
```

### Continuous Benchmarking

```bash
# Run benchmarks and save results
cargo bench -- --save-baseline main

# Compare against baseline
cargo bench -- --baseline main

# Run benchmarks with HTML reports
cargo bench -- --html

# Run benchmarks with custom output directory
cargo bench -- --output-dir ./bench_results
```

## Performance Monitoring

### Real-Time Performance Metrics

```rust
// examples/performance_monitoring.rs
use deribit_fix::*;
use std::time::Instant;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DeribitFixClient::new(Config::default());
    client.connect_and_authenticate().await?;
    
    let mut interval = interval(Duration::from_secs(1));
    
    loop {
        interval.tick().await;
        
        let start = Instant::now();
        
        // Perform operation
        let order = Order::new()
            .with_symbol("BTC-PERPETUAL")
            .with_side(OrderSide::Buy)
            .with_order_type(OrderType::Limit)
            .with_quantity(0.1)
            .with_price(50000.0);
        
        let result = client.place_order(order).await;
        let duration = start.elapsed();
        
        println!("Order placement took: {:?}", duration);
        
        if let Err(e) = result {
            println!("Error: {:?}", e);
        }
    }
}
```

### Performance Regression Detection

```rust
// examples/regression_detection.rs
use deribit_fix::*;
use std::collections::HashMap;

pub struct PerformanceMonitor {
    baseline_metrics: HashMap<String, f64>,
    current_metrics: HashMap<String, f64>,
    threshold: f64,
}

impl PerformanceMonitor {
    pub fn new(threshold: f64) -> Self {
        Self {
            baseline_metrics: HashMap::new(),
            current_metrics: HashMap::new(),
            threshold,
        }
    }
    
    pub fn set_baseline(&mut self, operation: &str, duration_ms: f64) {
        self.baseline_metrics.insert(operation.to_string(), duration_ms);
    }
    
    pub fn record_operation(&mut self, operation: &str, duration_ms: f64) {
        self.current_metrics.insert(operation.to_string(), duration_ms);
    }
    
    pub fn check_regressions(&self) -> Vec<String> {
        let mut regressions = Vec::new();
        
        for (operation, baseline) in &self.baseline_metrics {
            if let Some(current) = self.current_metrics.get(operation) {
                let degradation = (current - baseline) / baseline;
                if degradation > self.threshold {
                    regressions.push(format!(
                        "{}: {:.2}% degradation ({}ms -> {}ms)",
                        operation, degradation * 100.0, baseline, current
                    ));
                }
            }
        }
        
        regressions
    }
}
```

## Best Practices

### Benchmark Design

1. **Isolate Components**: Benchmark individual components separately
2. **Realistic Data**: Use realistic test data that represents production usage
3. **Multiple Scenarios**: Test different input sizes and scenarios
4. **Statistical Significance**: Ensure sufficient sample sizes for reliable results

### Performance Optimization

1. **Profile First**: Use profiling tools to identify bottlenecks
2. **Measure Changes**: Always measure performance impact of changes
3. **Avoid Premature Optimization**: Focus on bottlenecks identified by profiling
4. **Test Realistic Scenarios**: Benchmark with production-like workloads

### Continuous Monitoring

1. **Automated Benchmarks**: Run benchmarks in CI/CD pipeline
2. **Regression Detection**: Automatically detect performance regressions
3. **Trend Analysis**: Track performance over time
4. **Alerting**: Set up alerts for significant performance changes

## Next Steps

- [Mock Testing](./mock_testing.md) - Testing with external dependencies
- [Test Utilities](./test_utilities.md) - Common testing helpers
- [Unit Testing](./unit_testing.md) - Individual component testing
- [Integration Testing](./integration_testing.md) - Module interaction testing
