use crate::{utils, MAX_RECONNECT_ATTEMPTS, RECONNECT_DELAY, RECONNECT_INTERVAL};
use crate::{utils_connect, ImsBinanceDataIntegration};
use common_data_bar::TradeBar;
use common_data_bar_ext::SbeTradeBarExtension;
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{sleep, Instant};
use tokio_tungstenite::tungstenite::Message;
use trait_data_integration::{
    ImsDataIntegrationError, ImsSymbolIntegration, ImsTradeDataIntegration,
};
use trait_event_processor::EventProcessor;

impl ImsTradeDataIntegration for ImsBinanceDataIntegration {
    /// Starts real-time trade data streams for the specified symbols.
    ///
    /// This method establishes WebSocket connections for each symbol to receive real-time trade data.
    /// Each stream automatically reconnects every 12 hours to prevent server-side disconnections.
    /// If a connection fails, it will attempt to reconnect up to 5 times with a 5-second delay between attempts.
    ///
    /// # Process Flow
    /// 1. Validates all symbols before establishing connections
    /// 2. Creates a WebSocket connection for each symbol
    /// 3. Spawns an async task to process incoming trade data
    /// 4. Stores task handles for lifecycle management
    /// 5. Tracks active symbols for monitoring
    ///
    /// # Arguments
    /// * `symbols` - List of trading symbols (e.g., ["BTCUSDT", "ETHUSDT"])
    /// * `processor` - Event processor to handle incoming trade data
    ///
    /// # Returns
    /// - `Ok(())`: If all streams are started successfully
    /// - `Err(ImsDataIntegrationError)`: If symbol validation fails or connection errors occur
    ///
    /// # Connection Management
    /// - Automatic reconnection every 12 hours
    /// - Maximum 5 retry attempts on connection failure
    /// - 5-second delay between retry attempts
    /// - Graceful error handling with detailed logging
    ///
    async fn start_trade_data<P>(
        &self,
        symbols: &[String],
        processor: &Arc<P>,
    ) -> Result<(), ImsDataIntegrationError>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        // Validate symbols first
        self.validate_symbols(symbols).await?;

        let mut handlers = self.trade_handlers.write().await;
        let api_url = self.api_wss_url.clone();

        for symbol in symbols {
            let symbol = symbol.to_lowercase();

            if handlers.contains_key(&symbol) {
                // Symbol is already in the handlers collection, do nothing
                continue;
            }

            let stream_name = format!("{}@trade", symbol);
            let ws_stream =
                utils_connect::connect_websocket_static(&stream_name, api_url.clone()).await?;
            let processor = Arc::clone(processor);

            let symbol_clone = symbol.clone();
            let stream_name = format!("{}@trade", symbol_clone);
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
                                    let bar = utils::extract_trade_bar_from_json(
                                        text.as_str(),
                                        &symbol_clone,
                                    )
                                    .await;
                                    if let Some(bar) = bar {
                                        let (_, data) = TradeBar::encode_to_sbe(bar)
                                            .expect("Failed to encode trade data");
                                        if let Err(e) = processor.process_one_event(data).await {
                                            eprintln!(
                                                "Error processing trade data: {}",
                                                e.to_string()
                                            );
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
                                    "Successfully reconnected trade stream for {} (attempt {})",
                                    symbol_clone, retry_count
                                );
                                continue 'connection;
                            }
                            Err(e) => {
                                eprintln!(
                                    "Failed to reconnect trade stream for {}: {} (attempt {})",
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
            // Add symbol to active trade symbols list
            self.symbols_active_trade.write().await.push(symbol);
        }

        Ok(())
    }

    /// Stops real-time trade data streams for the specified symbols.
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
    /// - `Err(ImsDataIntegrationError)`: If any symbols are not found in active streams
    ///
    /// # Resource Management
    /// - Aborts WebSocket connection tasks
    /// - Cleans up handler storage
    /// - Updates active symbols tracking
    ///
    async fn stop_trade_data(&self, symbols: &[String]) -> Result<(), ImsDataIntegrationError> {
        // If no symbols provided, do nothing
        if symbols.is_empty() {
            return Ok(());
        }

        let mut handlers = self.trade_handlers.write().await;
        let mut stopped_symbols = Vec::new();
        let mut not_found_symbols = Vec::new();

        for symbol in symbols {
            let symbol = symbol.to_lowercase();

            // If symbol is not in trade_handlers, track it
            if !handlers.contains_key(&symbol) {
                not_found_symbols.push(symbol.clone());
                continue;
            }

            // Remove and abort the handler for this symbol
            if let Some(handle) = handlers.remove(&symbol) {
                handle.abort();
                stopped_symbols.push(symbol.clone());
                // Remove symbol from active trade symbols list
                let mut active_symbols = self.symbols_active_trade.write().await;
                if let Some(pos) = active_symbols.iter().position(|s| s == &symbol) {
                    active_symbols.remove(pos);
                }
            }
        }

        // If any symbols were not found in trade_handlers, return an error
        if !not_found_symbols.is_empty() {
            return Err(ImsDataIntegrationError::SymbolNotFoundError(format!(
                "The following symbols were not active trade streams: {:?}",
                not_found_symbols
            )));
        }

        Ok(())
    }

    /// Stops all active trade data streams.
    ///
    /// This method performs a complete shutdown of all active trade streams and cleans up all resources.
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
    /// - `Err(ImsDataIntegrationError)`: If cleanup fails
    ///
    /// # Resource Management
    /// - Aborts all WebSocket connection tasks
    /// - Clears handler storage
    /// - Clears active symbols tracking
    ///
    async fn stop_all_trade_data(&self) -> Result<(), ImsDataIntegrationError> {
        let mut handlers = self.trade_handlers.write().await;
        for (_, handle) in handlers.drain() {
            handle.abort();
        }

        // Clear trade handlers
        self.trade_handlers.write().await.clear();
        // Clear active trade symbols list when stopping all streams
        self.symbols_active_trade.write().await.clear();

        Ok(())
    }
}
