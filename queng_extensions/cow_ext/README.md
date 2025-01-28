# Copy-on-Write Extensions

## Overview

The `cow_ext` module provides efficient Copy-on-Write (CoW) extensions for string types in the Quant Engine project. It
focuses on optimizing memory usage through shallow cloning and efficient string handling.

## Components

### Clone Extension (`CloneExt`)

A trait providing efficient shallow cloning operations for string types.

#### Features:

- **Shallow Cloning**: Create borrowed views without deep copies
- **Memory Efficiency**: Avoid unnecessary allocations
- **Type Safety**: Guaranteed through Rust's lifetime system
- **Zero-Cost**: No runtime overhead for borrowed data

## Usage

### Basic Example

```rust
use std::borrow::Cow;
use cow_ext::CloneExt;

// With borrowed data
let borrowed = Cow::Borrowed("hello");
let cloned = borrowed.shallow_clone();
assert!(matches!(cloned, Cow::Borrowed(_)));

// With owned data
let owned: Cow<'_, str> = Cow::Owned(String::from("world"));
let cloned = owned.shallow_clone();
assert!(matches!(cloned, Cow::Borrowed(_)));
```

### Advanced Usage

```rust
use std::borrow::Cow;
use cow_ext::CloneExt;

fn process_string<'a>(input: Cow<'a, str>) -> Cow<'a, str> {
    // Create a borrowed view regardless of input variant
    let view = input.shallow_clone();
    
    // Use the borrowed view without allocation
    if view.len() > 10 {
        view
    } else {
        input
    }
}
```

## Design Principles

1. **Zero-Cost Abstractions**
    - No runtime overhead for borrowed data
    - Efficient memory usage
    - Compile-time optimizations

2. **Type Safety**
    - Strong lifetime guarantees
    - Safe borrowing patterns
    - Compile-time checks

3. **Performance**
    - Minimal allocations
    - Efficient string views
    - Memory-friendly operations

4. **Ergonomics**
    - Simple trait-based API
    - Clear documentation
    - Intuitive usage patterns

## Implementation Details

### Memory Management

- Uses Rust's `Cow` type for efficient string handling
- Avoids unnecessary allocations
- Preserves content while optimizing storage

### Safety Guarantees

- No unsafe code
- Memory safety through lifetime system
- Thread-safe operations

### Performance Optimizations

- Shallow cloning for efficiency
- Zero-copy operations where possible
- Minimal memory overhead

## Development

### Prerequisites

- Rust 1.84.0 or higher
- Standard library `std::borrow::Cow`

### Testing

```bash
cargo test
```

### Documentation

Generate documentation using:

```bash
cargo doc --no-deps --open
```

## Performance Considerations

1. **Memory Usage**
    - Efficient string views
    - Minimal allocations
    - Smart reference handling

2. **Runtime Performance**
    - Zero-cost abstractions
    - Optimal code generation
    - Efficient borrowing

3. **Resource Management**
    - Smart memory reuse
    - Efficient string handling
    - Minimal copying
