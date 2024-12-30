use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Implements the `ImsDataIntegration` trait for a struct containing an `ImsBinanceDataIntegration` field.
///
/// This derive macro automatically generates an implementation of the `ImsDataIntegration` trait
/// by delegating all trait methods to the struct's `integration` field, which must be of type
/// `ImsBinanceDataIntegration`.
///
/// # Requirements
/// - The struct must have a field named `integration` of type `ImsBinanceDataIntegration`
///
/// # Generated Methods
/// The following methods are implemented:
/// - `get_exchange_symbols() -> Result<HashSet<String>, MessageProcessingError>`
/// - `validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError>`
///
/// # Example
/// ```text
/// #[derive(BinanceImsDataIntegration)]
/// pub struct ImsBinanceSpotDataIntegration {
///     integration: ImsBinanceDataIntegration,
/// }
/// ```
///
/// This will generate an implementation that delegates all trait methods to the `integration` field.
#[proc_macro_derive(BinanceImsDataIntegration)]
pub fn derive_binance_ims_data_integration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl ImsDataIntegration for #name {
            async fn get_exchange_symbols(&self) -> Result<HashSet<String>, MessageProcessingError> {
                self.integration.get_exchange_symbols().await
            }

            async fn validate_symbols(&self, symbols: &[String]) -> Result<bool, MessageProcessingError> {
                self.integration.validate_symbols(symbols).await
            }
        }
    };

    TokenStream::from(expanded)
}

/// Implements the `ImsTradeDataIntegration` trait for a struct containing an `ImsBinanceDataIntegration` field.
///
/// This derive macro automatically generates an implementation of the `ImsTradeDataIntegration` trait
/// by delegating all trait methods to the struct's `integration` field, which must be of type
/// `ImsBinanceDataIntegration`.
///
/// # Requirements
/// - The struct must have a field named `integration` of type `ImsBinanceDataIntegration`
///
/// # Generated Methods
/// The following methods are implemented:
/// - `start_trade_data<P>(&self, symbols: &[String], processor: Arc<P>) -> Result<(), MessageProcessingError>`
/// - `stop_trade_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError>`
/// - `stop_all_trade_data(&self) -> Result<(), MessageProcessingError>`
///
/// Where `P: EventProcessor + Send + Sync + 'static`
///
/// # Example
/// ```text
/// #[derive(BinanceImsTradeDataIntegration)]
/// pub struct ImsBinanceSpotDataIntegration {
///     integration: ImsBinanceDataIntegration,
/// }
/// ```
///
/// This will generate an implementation that delegates all trait methods to the `integration` field.
#[proc_macro_derive(BinanceImsTradeDataIntegration)]
pub fn derive_binance_ims_trade_data_integration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl ImsTradeDataIntegration for #name {
            async fn start_trade_data<P>(
                &self,
                symbols: &[String],
                processor: Arc<P>,
            ) -> Result<(), MessageProcessingError>
            where
                P: EventProcessor + Send + Sync + 'static,
            {
                self.integration.start_trade_data(symbols, processor).await
            }

            async fn stop_trade_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError> {
                self.integration.stop_trade_data(symbols).await
            }

            async fn stop_all_trade_data(&self) -> Result<(), MessageProcessingError> {
                self.integration.stop_all_trade_data().await
            }
        }
    };

    TokenStream::from(expanded)
}

/// Implements the `ImsOhlcvDataIntegration` trait for a struct containing an `ImsBinanceDataIntegration` field.
///
/// This derive macro automatically generates an implementation of the `ImsOhlcvDataIntegration` trait
/// by delegating all trait methods to the struct's `integration` field, which must be of type
/// `ImsBinanceDataIntegration`.
///
/// # Requirements
/// - The struct must have a field named `integration` of type `ImsBinanceDataIntegration`
///
/// # Generated Methods
/// The following methods are implemented:
/// - `start_ohlcv_data<P>(&self, symbols: &[String], time_resolution: TimeResolution, processor: Arc<P>) -> Result<(), MessageProcessingError>`
/// - `stop_ohlcv_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError>`
/// - `stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError>`
///
/// Where `P: EventProcessor + Send + Sync + 'static`
///
/// # Example
/// ```text
/// #[derive(BinanceImsOhlcvDataIntegration)]
/// pub struct ImsBinanceSpotDataIntegration {
///     integration: ImsBinanceDataIntegration,
/// }
/// ```
///
/// This will generate an implementation that delegates all trait methods to the `integration` field.
#[proc_macro_derive(BinanceImsOhlcvDataIntegration)]
pub fn derive_binance_ims_ohlcv_data_integration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl ImsOhlcvDataIntegration for #name {
            async fn start_ohlcv_data<P>(
                &self,
                symbols: &[String],
                time_resolution: TimeResolution,
                processor: Arc<P>,
            ) -> Result<(), MessageProcessingError>
            where
                P: EventProcessor + Send + Sync + 'static,
            {
                self.integration
                    .start_ohlcv_data(symbols, time_resolution, processor)
                    .await
            }

            async fn stop_ohlcv_data(&self, symbols: &[String]) -> Result<(), MessageProcessingError> {
                self.integration.stop_ohlcv_data(symbols).await
            }

            async fn stop_all_ohlcv_data(&self) -> Result<(), MessageProcessingError> {
                self.integration.stop_all_ohlcv_data().await
            }
        }
    };

    TokenStream::from(expanded)
}
