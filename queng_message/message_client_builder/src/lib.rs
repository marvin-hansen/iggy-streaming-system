use iggy::clients::client::IggyClient;
use message_consumer::MessageConsumer;
use message_producer::MessageProducer;
use message_shared::{Args, IggyConfig};

mod build_fields;
mod builder;
mod error;
mod getters;

// Re-export error type
pub use crate::error::MessageClientBuilderError;
// Re-export the event consumer trait and Error type
pub use trait_event_consumer::EventConsumer;
pub use trait_event_consumer::EventConsumerError;

pub struct MessageClientBuilder {
    iggy_producer: MessageProducer,
    iggy_consumer: MessageConsumer,
}

impl MessageClientBuilder {
    /// Build a new `MessageClientBuilder` with the given iggy configuration.
    ///
    /// Assumptions:
    /// * The iggy client is connected and logged in after build.
    /// * The iggy producer and consumer using the same stream and topic.
    /// * The iggy consumer is set to consume last message by default i.e. no resend.
    ///
    /// If any of those assumptions or defaults don't fit your use case,
    /// consider using the from_args constructor instead.
    ///
    /// Args:
    /// * `iggy_config`: The configuration for the iggy client.
    ///
    /// Returns:
    /// A `Result` containing a tuple of:
    /// * A reference to the `IggyClient` created.
    /// * A reference to the `MessageClientBuilder` created.
    ///
    /// Errors:
    /// If the iggy client fails to build, a `MessageClientBuilderError` is returned.
    ///
    pub async fn new(
        iggy_config: &IggyConfig,
    ) -> Result<(IggyClient, Self), MessageClientBuilderError> {
        Self::build(false, Some(iggy_config), None).await
    }

    /// Build a new `MessageClientBuilder` with the given iggy configuration in debug mode.
    ///
    /// Assumptions:
    /// * The iggy client is connected and logged in after build.
    /// * The iggy producer and consumer using the same stream and topic.
    /// * The iggy consumer is set to consume last message by default i.e. no resend.
    ///
    /// If any of those assumptions or defaults don't fit your use case,
    /// consider using the from_args constructor instead.
    ///
    /// Args:
    /// * `iggy_config`: The configuration for the iggy client.
    ///
    /// Returns:
    /// A `Result` containing a tuple of:
    /// * A reference to the `IggyClient` created.
    /// * A reference to the `MessageClientBuilder` created.
    ///
    /// Errors:
    /// If the iggy client fails to build, a `MessageClientBuilderError` is returned.
    ///
    pub async fn with_debug(
        iggy_config: &IggyConfig,
    ) -> Result<(IggyClient, Self), MessageClientBuilderError> {
        Self::build(true, Some(iggy_config), None).await
    }

    /// Build a new custom `MessageClientBuilder` with the given arguments.
    ///
    /// Args:
    /// * `args`: The iggy arguments.
    ///
    /// Returns:
    /// A `Result` containing a tuple of:
    /// * A reference to the `IggyClient` created.
    /// * A reference to the `MessageClientBuilder` created.
    ///
    /// Errors:
    /// If the iggy client fails to build, a `MessageClientBuilderError` is returned.
    ///
    pub async fn from_args(
        args: Args,
        consumer_name: String,
    ) -> Result<(IggyClient, Self), MessageClientBuilderError> {
        Self::build(false, None, Some((args, consumer_name))).await
    }

    /// Build a new custom `MessageClientBuilder` with the given arguments in debug mode.
    ///
    /// Args:
    /// * `args`: The iggy arguments.
    ///
    /// Returns:
    /// A `Result` containing a tuple of:
    /// * A reference to the `IggyClient` created.
    /// * A reference to the `MessageClientBuilder` created.
    ///
    /// Errors:
    /// If the iggy client fails to build, a `MessageClientBuilderError` is returned.
    ///
    pub async fn from_args_with_debug(
        args: Args,
        consumer_name: String,
    ) -> Result<(IggyClient, Self), MessageClientBuilderError> {
        Self::build(true, None, Some((args, consumer_name))).await
    }
}
