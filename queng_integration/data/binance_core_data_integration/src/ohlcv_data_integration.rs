use crate::{utils, MAX_RECONNECT_ATTEMPTS, RECONNECT_DELAY, RECONNECT_INTERVAL};
use crate::{utils_connect, ImsBinanceDataIntegration};
use common_data_bar::{OHLCVBar, TimeResolution};
use common_data_bar_ext::SbeOHLCVBarExtension;
use common_errors::MessageProcessingError;
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{sleep, Instant};
use tokio_tungstenite::tungstenite::Message;
use trait_data_integration::{EventProcessor, ImsDataIntegration, ImsOhlcvDataIntegration};

impl ImsOhlcvDataIntegration for ImsBinanceDataIntegration {
    /// Starts real-time OHLCV (candlestick) data streams for the specified symbols.
    ///
    /// This method establishes WebSocket connections for each symbol to receive real-time OHLCV data.
    /// Each stream automatically reconnects every 12 hours to prevent server-side disconnections.
    /// If a connection fails, it will attempt to reconnect up to 5 times with a 5-second delay between attempts.
    ///
    /// # Process Flow
    /// 1. Validates all symbols before establishing connections
    /// 2. Creates a WebSocket connection for each symbol
    /// 3. Spawns an async task to process incoming OHLCV data
    /// 4. Stores task handles for lifecycle management
    /// 5. Tracks active symbols for monitoring
    ///
    /// # Arguments
    /// * `symbols` - List of trading symbols (e.g., ["BTCUSDT", "ETHUSDT"])
    /// * `time_resolution` - Time resolution for the candlesticks (e.g., 1m, 5m, 1h)
    /// * `processor` - Event processor to handle incoming OHLCV data
    ///
    /// # Returns
    /// - `Ok(())`: If all streams are started successfully
    /// - `Err(MessageProcessingError)`: If symbol validation fails or connection errors occur
    ///
    /// # Connection Management
    /// - Automatic reconnection every 12 hours
    /// - Maximum 5 retry attempts on connection failure
    /// - 5-second delay between retry attempts
    /// - Graceful error handling with detailed logging
    ///
    async fn start_ohlcv_data<P>(
        &self,
        symbols: &[String],
        time_resolution: TimeResolution,
        processor: Arc<P>,
    ) -> Result<(), MessageProcessingError>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        // Validate symbols first
        self.validate_symbols(symbols).await?;

        let mut handlers = self.ohlcv_handlers.write().await;
        let api_url = self.api_wss_url.clone();

        for symbol in symbols {
            let symbol = symbol.to_lowercase();

            if handlers.contains_key(&symbol) {
                // Symbol is already in the handlers collection, do nothing
                continue;
            }

            let stream_name = format!("{}@kline_{}", symbol, time_resolution);
            let ws_stream =
                utils_connect::connect_websocket_static(&stream_name, api_url.clone()).await?;
            let processor = Arc::clone(&processor);

            let symbol_clone = symbol.clone();
            let stream_name = format!("{}@kline_{}", symbol_clone, time_resolution);
            let api_url = api_url.clone();
            let handle = tokio::spawn(async move {
                let mut reconnect_time = Instant::now() + RECONNECT_INTERVAL;
                let mut ws_stream = ws_stream;

                'connection: loop {
                    loop {
                        // Check if we need to reconnect
                        if Instant::now() >= reconnect_time {
                            break;
                        }

                        // Process messages with timeout
                        match tokio::time::timeout(Duration::from_secs(1), ws_stream.next()).await {
                            Ok(Some(Ok(msg))) => {
                                if let Message::Text(text) = msg {
                                    let bar = utils::extract_ohlcv_bar_from_json(
                                        text.as_str(),
                                        &symbol_clone,
                                    )
                                    .await;
                                    if let Some(bar) = bar {
                                        let (_, data) = OHLCVBar::encode_to_sbe(bar)
                                            .expect("Failed to encode OHLCV data");
                                        if let Err(e) = processor.process(&[data]).await {
                                            eprintln!("Error processing OHLCV data: {}", e);
                                            return;
                                        }
                                    }
                                }
                            }
                            Ok(Some(Err(e))) => {
                                eprintln!("WebSocket error for {}: {}", symbol_clone, e);
                                break;
                            }
                            Ok(None) => {
                                eprintln!("WebSocket stream ended for {}", symbol_clone);
                                break;
                            }
                            Err(_) => continue, // Timeout, continue checking reconnect time
                        }
                    }

                    // Attempt to reconnect with retry limit
                    let mut retry_count = 0;
                    loop {
                        retry_count += 1;
                        match utils_connect::connect_websocket_static(&stream_name, api_url.clone())
                            .await
                        {
                            Ok(new_stream) => {
                                ws_stream = new_stream;
                                reconnect_time = Instant::now() + RECONNECT_INTERVAL;
                                eprintln!(
                                    "Successfully reconnected OHLCV stream for {} (attempt {})",
                                    symbol_clone, retry_count
                                );
                                continue 'connection;
                            }
                            Err(e) => {
                                eprintln!(
                                    "Failed to reconnect OHLCV stream for {}: {} (attempt {})",
                                    symbol_clone, e, retry_count
                                );
                                if retry_count >= MAX_RECONNECT_ATTEMPTS {
                                    eprintln!(
                                        "Max reconnection attempts ({}) reached for {}. Stopping stream.",
                                        MAX_RECONNECT_ATTEMPTS, symbol_clone
                                    );
                                    return;
                                }
                                // Wait before next retry
                                sleep(RECONNECT_DELAY).await;
                            }
                        }
                    }
                }
            });

            handlers.insert(symbol.clone(), handle);
            // Add symbol to active OHLCV symbols list
            self.symbols_active_ohlcv.write().await.push(symbol);
        }

        Ok(())
    }

    /// Stops real-time OHLCV data streams for the specified symbols.
    ///
    /// This method safely terminates WebSocket connections and cleans up resources for the specified symbols.
    /// It also maintains the list of active symbols by removing stopped streams from tracking.
    ///
    /// # Process Flow
    /// 1. Validates the input symbols list
    /// 2. Checks if each symbol has an active stream
    /// 3. Aborts the stream handler task
    /// 4. Removes the handler from storage
    /// 5. Updates the active symbols list
    ///
    /// # Arguments
    /// * `symbols` - List of trading symbols to stop (e.g., ["BTCUSDT", "ETHUSDT"])
    ///
    /// # Returns
    /// - `Ok(())`: If all specified streams are stopped successfully
    /// - `Err(MessageProcessingError)`: If any symbols are not found in active streams
    ///
    /// # Resource Management
    /// - Aborts WebSocket connection tasks
    /// - Cleans up handler storage
    /// - Updates active symbols tracking
    ///
    async fn stop_ohlcv_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError> {
        // If no symbols provided, do nothing
        if symbols.is_empty() {
            return Ok(());
        }

        let mut handlers = self.ohlcv_handlers.write().await;
        let mut stopped_symbols = Vec::new();
        let mut not_found_symbols = Vec::new();

        for symbol in symbols {
            let symbol = symbol.to_lowercase();

            // If symbol is not in ohlcv_handlers, track it
            if !handlers.contains_key(&symbol) {
                not_found_symbols.push(symbol.clone());
                continue;
            }

            // Remove and abort the handler for this symbol
            if let Some(handle) = handlers.remove(&symbol) {
                handle.abort();
                stopped_symbols.push(symbol.clone());
                // Remove symbol from active OHLCV symbols list
                let mut active_symbols = self.symbols_active_ohlcv.write().await;
                if let Some(pos) = active_symbols.iter().position(|s| s == &symbol) {
                    active_symbols.remove(pos);
                }
            }
        }

        // If any symbols were not found in ohlcv_handlers, return an error
        if !not_found_symbols.is_empty() {
            return Err(MessageProcessingError::new(format!(
                "The following symbols were not active OHLCV streams: {:?}",
                not_found_symbols
            )));
        }

        Ok(())
    }

    /// Stops all active OHLCV data streams.
    ///
    /// This method performs a complete shutdown of all active OHLCV streams and cleans up all resources.
    /// It's useful for application shutdown or when needing to reset all connections.
    ///
    /// # Process Flow
    /// 1. Acquires write lock on handlers
    /// 2. Aborts all stream handler tasks
    /// 3. Cleans up all handlers
    /// 4. Clears the active symbols list
    ///
    /// # Returns
    /// - `Ok(())`: If all streams are stopped successfully
    /// - `Err(MessageProcessingError)`: If cleanup fails
    ///
    /// # Resource Management
    /// - Aborts all WebSocket connection tasks
    /// - Clears handler storage
    /// - Clears active symbols tracking
    ///
    async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError> {
        let mut handlers = self.ohlcv_handlers.write().await;
        for (_, handle) in handlers.drain() {
            handle.abort();
            // Clear active OHLCV symbols list when stopping all streams
            self.symbols_active_ohlcv.write().await.clear();
        }
        Ok(())
    }
}
