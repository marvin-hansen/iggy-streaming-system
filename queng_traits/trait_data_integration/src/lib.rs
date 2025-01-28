// mod event_processor;
mod ims_data_error;
mod ims_ohlcv_data_integration;
mod ims_shutdown_integration;
mod ims_symbol_integration;
mod ims_trade_data_integration;

// pub use event_processor::EventProcessor;
pub use ims_data_error::ImsDataIntegrationError;
pub use ims_ohlcv_data_integration::ImsOhlcvDataIntegration;
pub use ims_shutdown_integration::ImsShutdownIntegration;
pub use ims_symbol_integration::ImsSymbolIntegration;
pub use ims_trade_data_integration::ImsTradeDataIntegration;

#[allow(dead_code)] // Clippy can't see that the trait is used
#[trait_variant::make(ImsDataIntegration: Send)]
pub trait LocalImsTradeDataIntegration:
    ImsSymbolIntegration + ImsTradeDataIntegration + ImsOhlcvDataIntegration + ImsShutdownIntegration
{
}
