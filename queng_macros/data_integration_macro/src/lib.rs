use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// A derive macro that implements the DataIntegrationTrait for a struct.
///
/// The struct must have two fields:
/// - `integration_id: ExchangeDataIntegrationID`
/// - `integration: ImsBinanceSpotDataIntegration`
///
/// # Example
/// ```text
/// use data_integration_macro::DataIntegration;
///
/// #[derive(DataIntegration)]
/// pub struct BinanceSpotDataIntegration {
///     // The struct must have the fields below to work
///     integration_id: ExchangeDataIntegrationID,
///     integration: ImsBinanceSpotDataIntegration,
/// }
/// ```
///
#[proc_macro_derive(DataIntegration)]
pub fn derive_data_integration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl DataIntegrationTrait for #name {
            async fn id(&self) -> Result<String, Error> {
                Ok(self.integration_id.to_string())
            }

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
