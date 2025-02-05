use iggy::client::{Client, UserClient};
use iggy::clients::client::IggyClient;
use message_consumer::MessageConsumer;
use message_producer::MessageProducer;
use message_shared::IggyConfig;

mod error;

// Re-export error type
pub use crate::error::MessageClientBuilderError;
// Re-export the event consumer trait and Error type
pub use trait_event_consumer::EventConsumer;
pub use trait_event_consumer::EventConsumerError;

pub struct MessageClientBuilder {
    iggy_config: IggyConfig,
    iggy_producer: MessageProducer,
    iggy_consumer: MessageConsumer,
}

impl MessageClientBuilder {
    /// Build a new `MessageClientBuilder` with the given iggy configuration.
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
        Self::build(false, iggy_config).await
    }

    /// Build a new `MessageClientBuilder` with the given iggy configuration and debug mode.
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
        Self::build(true, iggy_config).await
    }
}

impl MessageClientBuilder {
    /// Builds a new `MessageClientBuilder` with the given iggy configuration and debug mode.
    ///
    /// Args:
    /// * `dbg`: A boolean flag to enable debug printing.
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
    pub async fn build(
        dbg: bool,
        iggy_config: &IggyConfig,
    ) -> Result<(IggyClient, Self), MessageClientBuilderError> {
        let dbg_print = |msg: &str| {
            if dbg {
                println!("[MessageClientBuilder]: {msg}");
            }
        };

        let consumer_name = iggy_config.message_consumer_name();
        let stream_id = iggy_config.stream_name().to_string();
        let topic_id = iggy_config.topic_name().to_string();
        dbg_print(&format!("stream_id: {stream_id}"));
        dbg_print(&format!("topic_id: {topic_id}"));

        dbg_print("Build iggy client");
        let iggy_client = match message_shared::build_client(&iggy_config).await {
            Ok(client) => client,
            Err(err) => {
                return Err(MessageClientBuilderError::FailedToCreateIggyClient(
                    err.to_string(),
                ))
            }
        };

        dbg_print("Connect iggy client");
        match iggy_client.connect().await {
            Ok(_) => {}
            Err(err) => {
                return Err(MessageClientBuilderError::FailedToConnectToIggyServer(
                    format!(
                        "Failed to connect to control stream {} due to error : {}",
                        stream_id, err
                    ),
                ))
            }
        };

        dbg_print("Login iggy client");
        let username = iggy_config.user().username();
        let password = iggy_config.user().password();
        match iggy_client.login_user(username, password).await {
            Ok(_) => {}
            Err(err) => {
                return Err(MessageClientBuilderError::FailedToLoginIggyUser(format!(
                    "[ImsDataClient]: Failed to login user {} due to error: {}",
                    username, err
                )))
            }
        };

        let iggy_producer = match MessageProducer::from_client(
            dbg,
            &iggy_client,
            stream_id.clone(),
            topic_id.clone(),
        )
            .await
        {
            Ok(producer) => producer,
            Err(err) => {
                return Err(MessageClientBuilderError::FailedToCreateIggyProducer(
                    format!(
                        "Failed to create producer for stream: {} due to error {}",
                        stream_id, err
                    ),
                ))
            }
        };

        let iggy_consumer = match MessageConsumer::from_client(
            &iggy_client,
            consumer_name,
            stream_id.clone(),
            topic_id.clone(),
        )
            .await
        {
            Ok(consumer) => consumer,
            Err(err) => {
                return Err(MessageClientBuilderError::FailedToCreateIggyConsumer(
                    format!(
                        "Failed to create consumer for stream: {} due to error {}",
                        stream_id, err
                    ),
                ))
            }
        };

        Ok((
            iggy_client,
            Self {
                iggy_config: iggy_config.to_owned(),
                iggy_producer,
                iggy_consumer,
            },
        ))
    }
}

impl MessageClientBuilder {
    /// Returns a reference to the `IggyConfig` used to create the client.
    ///
    /// The `IggyConfig` contains the configuration for the iggy client,
    /// including the server address, username, password, and so on.
    pub fn iggy_config(&self) -> &IggyConfig {
        &self.iggy_config
    }
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
