# Environment Manager

## Overview

The `environment_manager` component provides environment detection and management capabilities for the Quant Engine
project. It ensures consistent environment handling across different deployment contexts and provides debugging
capabilities for environment-related issues.

## Core Features

### Environment Detection

- Automatic environment type detection
- Support for multiple environments:
    * LOCAL
    * CI
    * CLUSTER
    * UNKNOWN
- Debug mode for environment detection

### Environment Validation

- Environment variable validation
- Required variable checking
- Non-empty value validation

### Debug Support

- Debug mode for detailed logging
- Environment detection diagnostics
- Configuration validation logging

## Usage

### Basic Usage

```rust
use environment_manager::EnvironmentManager;

// Create default environment manager
let env_manager = EnvironmentManager::new();

// Get environment type
let env_type = env_manager.env_type();
```

### Debug Mode

```rust
use environment_manager::EnvironmentManager;

// Create environment manager with debug mode
let env_manager = EnvironmentManager::with_debug();

// Debug output will show:
// [EnvironmentManager]: Debug mode enabled
// [EnvironmentManager]: Detected environment type: LOCAL
```

## Environment Configuration

### Local Development

```bash
# Required environment variables
ENV=LOCAL
```

### CI Environment

```bash
# Required environment variables
ENV=CI
```

### Cluster Environment

```bash
# Required environment variables
ENV=CLUSTER
```

## Components

### Environment Manager (`lib.rs`)

- Core environment management functionality
- Environment type detection
- Debug mode support

### Utility Functions (`util.rs`)

- Environment variable validation
- Type detection implementation
- Debug message handling

## Design Principles

1. **Reliability**
    - Robust environment detection
    - Clear error messages
    - Fail-fast validation

2. **Simplicity**
    - Minimal configuration required
    - Clear environment types
    - Straightforward API

3. **Diagnostics**
    - Comprehensive debug mode
    - Clear error messages
    - Environment validation

4. **Safety**
    - Environment variable validation
    - Type-safe environment handling
    - Panic-free operation

## Development

### Prerequisites

- Rust 1.84.0 or higher
- Environment variables set according to context

### Testing

```bash
cargo test
```

### Debug Mode

Enable debug mode for detailed environment information:

```rust
let env_manager = EnvironmentManager::with_debug();
```

## Error Handling

The component handles several error conditions:

- Missing environment variables
- Invalid environment types
- Empty environment values

### Error Messages

- Clear and descriptive
- Context-specific
- Actionable recommendations

## Best Practices

1. **Environment Setup**
   ```bash
   # Always set the ENV variable
   export ENV=LOCAL  # or CI, CLUSTER
   ```

2. **Error Handling**
   ```rust
   // Always check environment type before operations
   match env_manager.env_type() {
       EnvironmentType::LOCAL => // handle local environment
       EnvironmentType::CI => // handle CI environment
       EnvironmentType::CLUSTER => // handle cluster environment
       EnvironmentType::UNKNOWN => // handle unknown environment
   }
   ```

3. **Debug Usage**
   ```rust
   // Use debug mode during development
   #[cfg(debug_assertions)]
   let env_manager = EnvironmentManager::with_debug();
   ```

## Performance Considerations

1. **Environment Detection**
    - One-time environment detection
    - Cached environment type
    - Minimal system calls

2. **Debug Mode**
    - Conditional debug output
    - No performance impact when disabled
    - Efficient logging implementation

3. **Validation**
    - Fast environment checks
    - Minimal memory usage
    - Efficient string comparisons
