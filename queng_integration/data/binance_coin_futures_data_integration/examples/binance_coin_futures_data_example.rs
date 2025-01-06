//! Example demonstrating the usage of Binance Coin Futures Data Integration
//!
//! This example shows how to:
//! 1. Create a Binance Coin Futures data integration instance
//! 2. Retrieve available symbols
//! 3. Validate symbols
//! 4. Start trade and OHLCV data streams
//! 5. Stop data streams

use binance_coin_futures_data_integration::ImsBinanceCoinFuturesDataIntegration;
use common_data_bar::{OHLCVBar, TimeResolution, TradeBar};
use common_data_bar_ext::{SbeOHLCVBarExtension, SbeTradeBarExtension};
use common_errors::MessageProcessingError;
use sbe_types::MessageType;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use trait_data_integration::{EventProcessor, ImsOhlcvDataIntegration, ImsSymbolIntegration, ImsTradeDataIntegration};

/// A simple event processor that prints received data to the console.
/// In a real application, you might want to parse the JSON and process
/// the data more comprehensively.
#[derive(Debug)]
struct PrintEventProcessor;

impl EventProcessor for PrintEventProcessor {
    async fn process(&self, data: &[Vec<u8>]) -> Result<(), MessageProcessingError> {
        let raw_message = data
            .first()
            .expect("Failed to get first element")
            .as_slice();
        // Determine SBE message type based on the second byte
        let message_type = MessageType::from(u16::from(raw_message[2]));

        // Decode and print SBE message relative to its message type
        match message_type {
            MessageType::TradeBar => {
                // SBE encoding and decoding is done via the SbeTradeBarExtension
                let bar = TradeBar::decode_from_sbe(raw_message)
                    .expect("Failed to decode trade bar message");

                println!("Received trade data:");
                println!("{}", bar);
            }
            MessageType::OHLCVBar => {
                // SBE encoding and decoding is done via the SbeOHLCVBarExtension
                let bar = OHLCVBar::decode_from_sbe(raw_message)
                    .expect("Failed to decode OHLCV bar message");
                println!("Received OHLCV data:");
                println!("{}", bar);
            }
            _ => {
                println!("Received unknown message type: {}", message_type);
            }
        }

        Ok(())
    }
}

/// Main example function demonstrating Binance Coin Futures data integration
#[tokio::main]
async fn main() -> Result<(), MessageProcessingError> {
    // Initialize rustls crypto provider for secure WebSocket connections
    // https://github.com/snapview/tokio-tungstenite/issues/353
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install default rustls crypto provider");

    // Create Binance Coin Futures data integration instance
    let integration = ImsBinanceCoinFuturesDataIntegration::new();
    let processor = Arc::new(PrintEventProcessor);

    // Retrieve and print available symbols
    let symbols = integration.get_exchange_symbols().await?;
    println!("Available Coin Futures Symbols: {:?}", symbols);

    // Select a few symbols to stream (ensure they exist)
    let test_symbols = vec!["BNBUSD_PERP".to_string(), "ETHUSD_PERP".to_string()];

    // Validate the symbols
    integration.validate_symbols(&test_symbols).await?;

    // Start trade data stream
    println!("\nStarting trade data stream...");
    if let Err(e) = integration
        .start_trade_data(&test_symbols, Arc::clone(&processor))
        .await
    {
        eprintln!("✗ Failed to start trade data stream: {}", e);
        return Err(e);
    }
    println!("✓ Trade data stream started successfully!");

    // Start OHLCV data stream
    println!("\nStarting OHLCV data stream...");
    if let Err(e) = integration
        .start_ohlcv_data(&test_symbols, TimeResolution::FiveMin, processor)
        .await
    {
        eprintln!("✗ Failed to start OHLCV data stream: {}", e);
        // Make sure to stop trade stream if OHLCV stream fails
        integration.stop_all_trade_data().await?;
        return Err(e);
    }
    println!("✓ OHLCV data stream started successfully!");

    // Run for a limited time to demonstrate streaming
    time::sleep(Duration::from_secs(10)).await;

    // Stop trade data stream
    if let Err(e) = integration.stop_all_trade_data().await {
        eprintln!("✗ Error stopping trade data stream: {}", e);
    }

    // Stop OHLCV data stream
    if let Err(e) = integration.stop_all_ohlcv_data().await {
        eprintln!("✗ Error stopping OHLCV data stream: {}", e);
    }

    println!("✓ All streams stopped successfully!");

    Ok(())
}
