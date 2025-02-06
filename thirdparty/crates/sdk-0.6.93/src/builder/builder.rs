use crate::builder::config::{Args, ConfigFields, IggyConfig};
use crate::builder::message_consumer::MessageConsumer;
use crate::builder::message_producer::MessageProducer;
use crate::builder::{utils, IggyBuilder};
use iggy::client::{Client, UserClient};
use iggy::clients::client::IggyClient;
use iggy::error::IggyError;
use tracing::error;

impl IggyBuilder {
    /// Builds an `IggyClient` and an `IggyBuilder` using the provided configuration or arguments.
    ///
    /// # Arguments
    ///
    /// * `iggy_config` - An optional `IggyConfig` to use for building the client.
    /// * `args` - An optional tuple of `Args` and a `String` to use for building the client.
    ///
    /// # Errors
    ///
    /// * `IggyError::InvalidConfiguration` - If no configuration or arguments are provided.
    /// * `IggyError::ConnectionError` - If a connection to the client could not be established.
    /// * `IggyError::AuthenticationError` - If authentication with the client fails.
    /// * `IggyError::InvalidIdentifier` - If the provided stream or topic identifier is invalid.
    /// * `IggyError::CommandLengthError` - If the length of a command is invalid.
    ///
    pub(crate) async fn build(
        iggy_config: Option<&IggyConfig>,
        args: Option<(Args, String)>,
    ) -> Result<(IggyClient, Self), IggyError> {
        if iggy_config.is_none() && args.is_none() {
            error!("Configuration missing. Please provide a config to build iggy client.");
            return Err(IggyError::InvalidConfiguration);
        }

        let config_fields = if let Some(iggy_config) = iggy_config {
            ConfigFields::from_iggy_config(iggy_config)
        } else {
            let (args, consumer_name) = args.clone().unwrap();
            ConfigFields::from_args(args, consumer_name)
        };

        let consumer_name = config_fields.consumer_name();
        let stream_id = config_fields.stream_id().to_string();
        let topic_id = config_fields.topic_id().to_string();
        let username = config_fields.username();
        let password = config_fields.password();

        let iggy_client = if let Some(iggy_config) = iggy_config {
            match utils::build_tcp_client_from_config(iggy_config).await {
                Ok(client) => client,
                Err(err) => {
                    error!("Failed to create iggy client: {}", err);
                    return Err(err);
                }
            }
        } else if let Some((args, _)) = args {
            match utils::build_tcp_client_from_args(args).await {
                Ok(client) => client,
                Err(err) => {
                    error!("Failed to create iggy client: {}", err);
                    return Err(err);
                }
            }
        } else {
            error!("Configuration missing. Please provide a config to build iggy client.");
            return Err(IggyError::InvalidConfiguration);
        };

        match iggy_client.connect().await {
            Ok(_) => {}
            Err(err) => return Err(err),
        };

        match iggy_client.login_user(username, password).await {
            Ok(_) => {}
            Err(err) => return Err(err),
        };

        let iggy_producer =
            match Self::build_producer(&iggy_client, stream_id.clone(), topic_id.clone()).await {
                Ok(producer) => producer,
                Err(err) => return Err(err),
            };

        let iggy_consumer =
            match Self::build_consumer(&iggy_client, consumer_name, stream_id, topic_id).await {
                Ok(consumer) => consumer,
                Err(err) => return Err(err),
            };

        Ok((
            iggy_client,
            Self {
                iggy_producer,
                iggy_consumer,
            },
        ))
    }

    /// Builds a `MessageConsumer` using the provided `IggyClient` and identifiers.
    ///
    /// # Arguments
    ///
    /// * `iggy_client` - The `IggyClient` to use for authentication and communication.
    /// * `consumer_name` - The name of the consumer.
    /// * `stream_id` - The identifier of the stream.
    /// * `topic_id` - The identifier of the topic.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageConsumer` instance or an `IggyError`.
    ///
    /// # Errors
    ///
    /// * `IggyError::InvalidIdentifier` - If the provided stream or topic identifier is invalid.
    /// * `IggyError::AuthenticationError` - If authentication with the client fails.
    /// * `IggyError::ConnectionError` - If a connection to the client could not be established.
    /// * `IggyError::CommandLengthError` - If the length of a command is invalid.
    ///
    pub async fn build_consumer(
        iggy_client: &IggyClient,
        consumer_name: &str,
        stream_id: String,
        topic_id: String,
    ) -> Result<MessageConsumer, IggyError> {
        MessageConsumer::from_client(&iggy_client, consumer_name, stream_id, topic_id).await
    }

    /// Builds a `MessageProducer` using the provided `IggyClient` and identifiers.
    ///
    /// # Arguments
    ///
    /// * `iggy_client` - The `IggyClient` to use for authentication and communication.
    /// * `stream_id` - The identifier of the stream.
    /// * `topic_id` - The identifier of the topic.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageProducer` instance or an `IggyError`.
    ///
    /// # Errors
    ///
    /// * `IggyError::InvalidIdentifier` - If the provided stream or topic identifier is invalid.
    /// * `IggyError::AuthenticationError` - If authentication with the client fails.
    /// * `IggyError::ConnectionError` - If a connection to the client could not be established.
    /// * `IggyError::CommandLengthError` - If the length of a command is invalid.
    ///
    pub async fn build_producer(
        iggy_client: &IggyClient,
        stream_id: String,
        topic_id: String,
    ) -> Result<MessageProducer, IggyError> {
        MessageProducer::from_client(&iggy_client, stream_id, topic_id).await
    }
}

// Getters
impl IggyBuilder {
    /// Returns a reference to the `MessageProducer` created for this client.
    pub fn iggy_producer(&self) -> &MessageProducer {
        &self.iggy_producer
    }
    /// Returns the `MessageConsumer` created for this client.
    ///
    /// Note that this method consumes `self`.
    pub fn iggy_consumer(self) -> MessageConsumer {
        self.iggy_consumer
    }
}
