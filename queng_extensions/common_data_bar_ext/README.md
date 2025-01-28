# Common Data Bar Extensions

## Overview

The `common_data_bar_ext` module provides Simple Binary Encoding (SBE) extensions for market data bar types in the Quant
Engine project. These extensions enable efficient binary serialization and deserialization of OHLCV and Trade bar data.

## Components

### OHLCV Bar Extensions (`ohlcv_bar_ext`)

Extends the `OHLCVBar` type with SBE encoding and decoding capabilities.

#### Features:

- **SBE Encoding**: Convert OHLCV bars to binary format
- **SBE Decoding**: Parse binary data back to OHLCV bars
- **Zero-copy deserialization**: Efficient parsing of binary data
- **Error handling**: Comprehensive encode/decode error types

### Trade Bar Extensions (`trade_bar_ext`)

Extends the `TradeBar` type with SBE encoding and decoding capabilities.

#### Features:

- **SBE Encoding**: Convert trade bars to binary format
- **SBE Decoding**: Parse binary data back to trade bars
- **Zero-copy deserialization**: Efficient parsing of binary data
- **Error handling**: Comprehensive encode/decode error types

## Usage

### OHLCV Bar Example

```rust
use common_data_bar::OHLCVBar;
use common_data_bar_ext::SbeOHLCVBarExtension;

// Create an OHLCV bar
let ohlcv_bar = OHLCVBar::new(/* ... */);

// Encode to SBE binary format
let (size, buffer) = ohlcv_bar.encode_to_sbe()?;

// Decode from SBE binary format
let decoded_bar = OHLCVBar::decode_from_sbe(&buffer)?;
```

### Trade Bar Example

```rust
use common_data_bar::TradeBar;
use common_data_bar_ext::SbeTradeBarExtension;

// Create a trade bar
let trade_bar = TradeBar::new(/* ... */);

// Encode to SBE binary format
let (size, buffer) = trade_bar.encode_to_sbe()?;

// Decode from SBE binary format
let decoded_bar = TradeBar::decode_from_sbe(&buffer)?;
```

## Error Handling

The extensions provide two main error types:

- `SbeEncodeError`: Errors that occur during encoding
- `SbeDecodeError`: Errors that occur during decoding

## Design Principles

1. **Zero-Cost Abstractions**
    - Extensions compile to optimal machine code
    - No runtime overhead
    - Efficient memory usage

2. **Type Safety**
    - Strong type checking at compile time
    - Clear error types
    - Safe binary parsing

3. **Performance**
    - Zero-copy deserialization where possible
    - Minimal allocations
    - Efficient binary encoding

4. **Ergonomics**
    - Simple trait-based API
    - Consistent error handling
    - Clear documentation

## Integration with SBE

These extensions integrate with the `sbe_messages_data` crate to provide:

- Standard compliant SBE encoding
- Efficient binary message formats
- Consistent serialization across the system

## Development

### Prerequisites

- Rust 1.84.0 or higher
- `common_data_bar` crate
- `sbe_messages_data` crate
- `sbe_types` crate

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

1. **Binary Format**
    - Compact representation
    - Fast serialization/deserialization
    - Cache-friendly layout

2. **Memory Usage**
    - Minimal temporary allocations
    - Efficient buffer management
    - Zero-copy where possible

3. **Error Handling**
    - Fast error paths
    - Clear error messages
    - No panic conditions

