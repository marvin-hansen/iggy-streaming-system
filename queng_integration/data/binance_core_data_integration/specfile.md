# Binance Core Data Integration Requirements

## Overview
The `binance_core_data_integration` crate provides a robust implementation for real-time market data integration with the Binance cryptocurrency exchange. It supports streaming of both trade and OHLCV (candlestick) data through WebSocket connections, with built-in connection management, automatic reconnection, and error handling capabilities.

## Functional Requirements

### Core Components

#### ImsBinanceDataIntegration
The main struct that implements data integration functionality.

```rust
pub struct ImsBinanceDataIntegration {
    api_base_url: String,
    api_wss_url: String,
    http_client: Client,
    symbols_active_trade: RwLock<Vec<String>>,
    symbols_active_ohlcv: RwLock<Vec<String>>,
    symbol_cache: RwLock<Option<(HashSet<String>, Instant)>>,
    trade_handlers: RwLock<HashMap<String, JoinHandle<()>>>,
    ohlcv_handlers: RwLock<HashMap<String, JoinHandle<()>>>,
}
```

### Trade Data Integration

#### Methods

1. `start_trade_data<P>(&self, symbols: &[String], processor: Arc<P>) -> Result<(), MessageProcessingError>`
   - Starts real-time trade data streams for specified symbols
   - Parameters:
     - `symbols`: List of trading symbols (e.g., ["BTCUSDT", "ETHUSDT"])
     - `processor`: Event processor to handle incoming trade data
   - Returns: Result indicating success or failure

2. `stop_trade_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError>`
   - Stops trade data streams for specified symbols
   - Parameters:
     - `symbols`: List of trading symbols to stop
   - Returns: Result indicating success or failure

3. `stop_all_trade_data(&self) -> Result<(), MessageProcessingError>`
   - Stops all active trade data streams
   - Returns: Result indicating success or failure

### OHLCV Data Integration

#### Methods

1. `start_ohlcv_data<P>(&self, symbols: &[String], time_resolution: TimeResolution, processor: Arc<P>) -> Result<(), MessageProcessingError>`
   - Starts real-time OHLCV data streams for specified symbols
   - Parameters:
     - `symbols`: List of trading symbols
     - `time_resolution`: Time resolution for candlesticks (e.g., 1m, 5m, 1h)
     - `processor`: Event processor to handle incoming OHLCV data
   - Returns: Result indicating success or failure

2. `stop_ohlcv_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError>`
   - Stops OHLCV data streams for specified symbols
   - Parameters:
     - `symbols`: List of trading symbols to stop
   - Returns: Result indicating success or failure

3. `stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError>`
   - Stops all active OHLCV data streams
   - Returns: Result indicating success or failure

## Non-Functional Requirements

### Performance
1. Efficient WebSocket connection management
2. Minimal latency in data processing
3. Thread-safe operations using `RwLock`
4. Asynchronous processing using Tokio runtime

### Reliability
1. Automatic reconnection mechanism
2. Connection health monitoring
3. Error handling and recovery
4. Symbol validation with caching

### Configuration
1. Configurable reconnection intervals (default: 12 hours)
2. Configurable retry attempts (default: 5)
3. Configurable retry delay (default: 5 seconds)
4. Configurable symbol cache duration (default: 120 minutes)

### Resource Management
1. Efficient cleanup of terminated connections
2. Memory-efficient symbol caching
3. Proper resource allocation and deallocation

## Implementation Notes

### WebSocket Connection Management
1. Each symbol stream runs in a separate async task
2. Automatic reconnection every 12 hours to prevent server-side disconnections
3. Maximum 5 retry attempts on connection failure
4. 5-second delay between retry attempts

### Data Processing
1. Trade and OHLCV data are processed using Simple Binary Encoding (SBE)
2. Event processors must implement the `EventProcessor` trait
3. Data is validated before being passed to processors
4. Efficient JSON parsing for incoming WebSocket messages

### Symbol Management
1. Symbols are cached to reduce API calls
2. Cache is refreshed every 120 minutes
3. All symbols are validated before establishing connections
4. Case-insensitive symbol handling (converted to lowercase)

### Error Handling
1. Comprehensive error types through `MessageProcessingError`
2. Graceful connection termination
3. Detailed error logging
4. Recovery mechanisms for common failure scenarios

### Adapting for Other Exchanges
To adapt this implementation for another exchange:

1. Create a new struct implementing the same traits (`ImsDataIntegration`, `ImsTradeDataIntegration`, `ImsOhlcvDataIntegration`)
2. Modify WebSocket connection URLs and endpoints
3. Adjust JSON parsing in utils.rs for exchange-specific message formats
4. Update symbol validation logic according to exchange requirements
5. Modify reconnection strategy if needed for exchange-specific requirements
6. Implement exchange-specific error handling
7. Update data encoding/decoding if the exchange uses different formats

## Detailed WebSocket Connection Management

### Connection Establishment
1. Each symbol stream is established through a dedicated WebSocket connection
2. Connection URL format: `{base_ws_url}/{stream_name}`
   - For trades: `{symbol}@trade`
   - For OHLCV: `{symbol}@kline_{time_resolution}`

### Connection Lifecycle
1. Initial Connection:
   - Establishes WebSocket connection using `tokio_tungstenite`
   - Spawns dedicated async task for each stream
   - Sets initial reconnect time to current time + 12 hours

2. Message Processing:
   - Uses 1-second timeout for message polling
   - Processes messages in a continuous loop
   - Deserializes JSON messages into trade or OHLCV bars
   - Encodes data using Simple Binary Encoding (SBE)
   - Forwards processed data to the event processor

3. 12-Hour Reconnection Strategy:
   ```rust
   let mut reconnect_time = Instant::now() + RECONNECT_INTERVAL; // 12 hours
   'connection: loop {
       loop {
           // Check if we need to reconnect
           if Instant::now() >= reconnect_time {
               break;
           }
           // Process messages...
       }
       // Attempt reconnection
   }
   ```

### Reconnection Process
1. Scheduled Reconnection (Every 12 Hours):
   - Purpose: Prevent server-side connection termination
   - Timing: Controlled by `RECONNECT_INTERVAL` (12 hours)
   - Process:
     a. Break from message processing loop
     b. Close existing connection
     c. Establish new connection
     d. Reset reconnect timer

2. Error-Triggered Reconnection:
   - Maximum Attempts: 5 (controlled by `MAX_RECONNECT_ATTEMPTS`)
   - Delay Between Attempts: 5 seconds (`RECONNECT_DELAY`)
   - Process:
     a. Log connection error
     b. Wait for delay period
     c. Attempt to establish new connection
     d. If successful, resume message processing
     e. If failed after max attempts, terminate handler

### Connection States
1. Active Processing:
   - Continuously polling for messages
   - Processing incoming data
   - Monitoring reconnect timer

2. Reconnection:
   - Attempting to establish new connection
   - Waiting between retry attempts
   - Validating new connection

3. Terminated:
   - Maximum retry attempts exceeded
   - Manual stop requested
   - Unrecoverable error occurred

### Error Handling
1. Connection Errors:
   - Network connectivity issues
   - Invalid stream names
   - Server rejections

2. Message Processing Errors:
   - Invalid JSON format
   - Unexpected message structure
   - Encoding/decoding failures

3. Recovery Actions:
   - Automatic retry with exponential backoff
   - Detailed error logging
   - Resource cleanup on failure

### Resource Management
1. Connection Cleanup:
   - Proper WebSocket connection closure
   - Task cancellation on shutdown
   - Memory cleanup for terminated handlers

2. Memory Considerations:
   - Efficient message buffer management
   - Proper cleanup of processed messages
   - Prevention of memory leaks in long-running connections

## State Management and Thread Safety

### Core State Components

1. Symbol Management:
   ```rust
   symbols_active_trade: RwLock<Vec<String>>
   symbols_active_ohlcv: RwLock<Vec<String>>
   ```
   - Thread-safe tracking of active symbols using `RwLock`
   - Separate vectors for trade and OHLCV streams
   - Pre-allocated capacity of 50 symbols for performance
   - Multiple readers, single writer access pattern

2. Symbol Cache:
   ```rust
   symbol_cache: RwLock<Option<(HashSet<String>, Instant)>>
   ```
   - Thread-safe caching of valid exchange symbols
   - Tuple stores symbols and cache timestamp
   - Cache invalidation after 120 minutes
   - Lazy loading pattern - only fetched when needed
   - `HashSet` for O(1) symbol validation

3. Connection Handlers:
   ```rust
   trade_handlers: RwLock<HashMap<String, JoinHandle<()>>>
   ohlcv_handlers: RwLock<HashMap<String, JoinHandle<()>>>
   ```
   - Thread-safe storage of WebSocket connection tasks
   - Key: Symbol name (lowercase)
   - Value: Tokio task handle for the connection
   - Pre-allocated capacity of 50 handlers
   - Enables individual stream management

### Thread Safety Mechanisms

1. Read-Write Locks (`RwLock`):
   - Multiple readers OR single writer access
   - Prevents data races and concurrent modifications
   - Async-aware locks from Tokio
   - Granular locking per component
   - Deadlock prevention through consistent lock ordering

2. State Access Patterns:
   ```rust
   // Read access example
   let symbols = self.symbols_active_trade.read().await;
   
   // Write access example
   let mut handlers = self.trade_handlers.write().await;
   handlers.insert(symbol.clone(), handle);
   ```
   - Asynchronous lock acquisition
   - Scope-based lock release
   - Minimal lock hold times
   - Error-resilient lock handling

3. Symbol Validation Flow:
   ```rust
   async fn validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError> {
       let valid_symbols = self.get_exchange_symbols().await?;
       // Read-only access to cached symbols
       let invalid_symbols: Vec<_> = symbols
           .iter()
           .filter(|s| !valid_symbols.contains(*s))
           .collect();
       // ...
   }
   ```
   - Thread-safe symbol validation
   - Cache-first approach
   - Atomic operations
   - Error handling with proper cleanup

### State Lifecycle Management

1. Symbol State Updates:
   - Atomic addition of new symbols
   - Safe removal of inactive symbols
   - Consistent state between handlers and active symbols
   - Proper cleanup on connection termination

2. Handler Lifecycle:
   - Safe task spawning and storage
   - Proper task abortion on shutdown
   - Resource cleanup after task completion
   - Error handling with state consistency

3. Cache Management:
   - Atomic cache updates
   - Time-based invalidation
   - Lazy refresh mechanism
   - Thread-safe cache access

### Concurrency Benefits

1. Performance:
   - Multiple simultaneous readers
   - Minimal contention on read operations
   - Efficient symbol validation
   - Quick handler lookups

2. Safety:
   - No data races
   - Consistent state
   - Safe concurrent access
   - Proper resource cleanup

3. Scalability:
   - Independent stream management
   - Parallel data processing
   - Efficient memory usage
   - Controllable resource allocation

### Implementation Considerations

1. Lock Granularity:
   - Separate locks for different components
   - Minimal lock scope
   - Quick lock release
   - Deadlock prevention

2. Error Handling:
   - State consistency during errors
   - Proper cleanup on failures
   - Error propagation
   - Recovery mechanisms

3. Resource Management:
   - Memory-efficient data structures
   - Proper task cleanup
   - Cache size control
   - Connection pooling
