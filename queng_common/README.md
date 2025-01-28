# Quant Engine Common Libraries

## Overview

The `queng_common` module provides a comprehensive set of common libraries and utilities for the Quant Engine project.
It serves as the foundation for shared functionality across the entire system, ensuring consistency and reusability.

## Components

### Exchange Management (`common_exchange`)

- Exchange identification and management
- Symbol and instrument definitions
- Portfolio configuration
- Account type management
- Security type definitions
- Fiat currency ISO code handling
- Master symbol management

### Data Structures (`common_data_bar`)

- OHLCV (Open, High, Low, Close, Volume) bar implementations
- Trade bar handling
- Time resolution management
- Sampled bar processing
- Display and getter utilities for data visualization

### Configuration (`common_config`)

- System-wide configuration management
- Environment-specific settings
- Configuration validation and parsing

### Database (`common_database`)

- Database connection management
- Query utilities
- Data persistence layer

### Environment (`common_env`)

- Environment variable management
- Runtime environment configuration
- System settings

### Error Handling (`common_errors`)

- Comprehensive error type hierarchy:
    - Database gateway errors
    - Download operation errors
    - Initialization errors
    - Lookup errors
    - Message processing errors
    - PostgreSQL specific errors
    - Input validation errors
    - Message client configuration errors
    - Data sanitization errors
- Error propagation and handling utilities
- Error handling patterns

### Integration Management System (`common_ims`)

- Integration system core functionality
- External system integration management
- Integration configuration and control

### Message Handling (`common_message`)

- Message type definitions
- Message processing utilities
- Communication protocols

### Metadata Management (`common_metadata`)

- Metadata definitions
- Metadata processing
- System information management

### Order Management (`common_order`)

- Order type definitions
- Order processing utilities
- Order state management

### Orderbook (`common_orderbook`)

- L2 orderbook implementation with separate bid/ask handling

### Service Management (`common_service`)

- Service lifecycle management (startup/shutdown utilities)
- Print utilities for service logging and debugging
- Graceful shutdown handling
- Service status monitoring

### Trade Management (`common_trade`)

- Trade strategy configuration and management
- Pattern-based trading support
- Trade direction handling (long/short)
- Trade entry management
- Strategy configuration utilities
- Pattern type definitions and configurations

## Usage

Each component can be included individually in your Cargo.toml:

```toml
[dependencies]
common_exchange = { path = "path/to/queng_common/common_exchange" }
common_data_bar = { path = "path/to/queng_common/common_data_bar" }
# ... other components as needed
```

## Design Principles

1. **Modularity**: Each component is self-contained and can be used independently
2. **Consistency**: Common patterns and practices across all components
3. **Type Safety**: Strong type system ensuring compile-time correctness
4. **Performance**: Optimized for high-performance financial applications
5. **Reusability**: Shared functionality to prevent code duplication

## Development

### Prerequisites

- Rust 1.84.0 or higher
- Cargo build system
- Required development dependencies

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

## Documentation

Each component contains its own documentation. Generate documentation using:

```bash
cargo doc --no-deps --open
```

## License

See the project's main LICENSE file for licensing information.
