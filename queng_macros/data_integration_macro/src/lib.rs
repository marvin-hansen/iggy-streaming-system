//! Procedural macros for implementing IMS data integration traits.
//! 
//! This crate provides derive macros that automatically implement data integration
//! traits for structs containing an IMS integration field.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Automatically implements IMS data integration traits for a struct.
///
/// This derive macro generates implementations for the following traits:
/// - `ImsDataIntegration`
/// - `ImsSymbolIntegration`
/// - `ImsTradeDataIntegration` 
/// - `ImsOhlcvDataIntegration`
///
/// # Requirements
///
/// The struct must have a field named `integration` of type `ImsBinanceDataIntegration`.
/// All trait method implementations will delegate to this field.
///
/// # Example
///
/// ```text
/// use data_integration_macro::ImsDataIntegration;
///
/// #[derive(ImsDataIntegration)]
/// pub struct ImsBinanceSpotDataIntegration {
///     integration: ImsBinanceDataIntegration,
/// }
/// ```
///
/// This will generate implementations for all required traits, with methods
/// delegating to the corresponding methods on the `integration` field.
///
/// # Generated Implementations
///
/// The macro generates the following trait implementations:
///
/// ```text
/// impl ImsDataIntegration for ImsBinanceSpotDataIntegration {}
///
/// impl ImsSymbolIntegration for ImsBinanceSpotDataIntegration {
///     // Methods delegating to self.integration
/// }
///
/// impl ImsTradeDataIntegration for ImsBinanceSpotDataIntegration {
///     // Methods delegating to self.integration
/// }
///
/// impl ImsOhlcvDataIntegration for ImsBinanceSpotDataIntegration {
///     // Methods delegating to self.integration
/// }
/// ```
#[proc_macro_derive(ImsDataIntegrationImpl)]
pub fn ims_data_integration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {

        impl ImsDataIntegration for #name {}

        impl ImsSymbolIntegration for #name {
            fn get_exchange_symbols(
                &self,
            ) -> impl Future<Output = Result<HashSet<String>, MessageProcessingError>> + Send {
                self.integration.get_exchange_symbols()
            }

            fn validate_symbols(
                &self,
                symbols: &[String],
            ) -> impl Future<Output = Result<bool, MessageProcessingError>> + Send {
                self.integration.validate_symbols(symbols)
            }
        }

        impl ImsTradeDataIntegration for #name {
            fn start_trade_data<P>(
                &self,
                symbols: &[String],
                processor: Arc<P>,
            ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send
            where
                P: EventProcessor + Send + Sync + 'static,
            {
                self.integration.start_trade_data(symbols, processor)
            }

            fn stop_trade_data(
                &self,
                symbols: &[String],
            ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send {
                self.integration.stop_trade_data(symbols)
            }

            fn stop_all_trade_data(
                &self,
            ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send {
                self.integration.stop_all_trade_data()
            }
        }

        impl ImsOhlcvDataIntegration for #name {
            fn start_ohlcv_data<P>(
                &self,
                symbols: &[String],
                time_resolution: TimeResolution,
                processor: Arc<P>,
            ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send
            where
                P: EventProcessor + Send + Sync + 'static,
            {
                self.integration
                    .start_ohlcv_data(symbols, time_resolution, processor)
            }

            fn stop_ohlcv_data(
                &self,
                symbols: &[String],
            ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send {
                self.integration.stop_ohlcv_data(symbols)
            }

            fn stop_all_ohlcv_data(
                &self,
            ) -> impl Future<Output = Result<(), MessageProcessingError>> + Send {
                self.integration.stop_all_ohlcv_data()
            }
        }
    };

    TokenStream::from(expanded)
}
