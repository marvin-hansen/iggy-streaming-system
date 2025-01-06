use binance_spot_data_integration::ImsBinanceSpotDataIntegration;
use common_data_bar::{OHLCVBar, TimeResolution, TradeBar};
use common_data_bar_ext::{SbeOHLCVBarExtension, SbeTradeBarExtension};
use common_errors::MessageProcessingError;
use sbe_types::MessageType;
use std::sync::Arc;
use tokio::time::Duration;
use trait_data_integration::{EventProcessor, ImsOhlcvDataIntegration, ImsSymbolIntegration, ImsTradeDataIntegration};

/// A simple event processor that prints received data to the console.
/// In a real application, you might want to parse the JSON and process
/// the data according to your needs.
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
                let bar = TradeBar::decode_from_sbe(raw_message)
                    .expect("Failed to decode trade bar message");
                println!("Received trade data:");
                println!("{}", bar);
            }
            MessageType::OHLCVBar => {
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

/// This example demonstrates how to use the Binance data integration to:
/// 1. Validate trading symbols
/// 2. Start real-time trade data streams
/// 3. Start real-time OHLCV data streams
/// 4. Process incoming data
/// 5. Gracefully stop all streams
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize rustls crypto provider for secure WebSocket connections
    // https://github.com/snapview/tokio-tungstenite/issues/353
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install default rustls crypto provider");

    // Create a new instance of the Binance integration
    let integration = ImsBinanceSpotDataIntegration::new();
    let processor = Arc::new(PrintEventProcessor);

    // Example symbols - you can modify these to any valid Binance trading pairs
    let symbols = vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()];

    // Step 1: Validate symbols
    println!("Validating symbols...");
    match integration.validate_symbols(&symbols).await {
        Ok(_) => println!("✓ Symbols validated successfully!"),
        Err(e) => {
            eprintln!("✗ Symbol validation failed: {}", e);
            return Err(e.into());
        }
    }

    // Step 2: Start trade data stream
    println!("\nStarting trade data stream...");
    if let Err(e) = integration
        .start_trade_data(&symbols, Arc::clone(&processor))
        .await
    {
        eprintln!("✗ Failed to start trade data stream: {}", e);
        return Err(e.into());
    }
    println!("✓ Trade data stream started successfully!");

    // Step 3: Start OHLCV data stream
    println!("\nStarting OHLCV data stream...");
    if let Err(e) = integration
        .start_ohlcv_data(&symbols, TimeResolution::OneMin, processor)
        .await
    {
        eprintln!("✗ Failed to start OHLCV data stream: {}", e);
        // Make sure to stop trade stream if OHLCV stream fails
        integration.stop_all_trade_data().await?;
        return Err(e.into());
    }
    println!("✓ OHLCV data stream started successfully!");

    // Step 4: Let the streams run for 10 seconds
    println!("\nStreams are now running. Waiting for 10 seconds...");
    println!("You should see trade and OHLCV data being printed below:\n");
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Step 5: Gracefully stop all streams
    println!("\nStopping all streams...");

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
