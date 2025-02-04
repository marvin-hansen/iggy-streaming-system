use iggy::client::{Client, UserClient};
use iggy::clients::client::IggyClient;
use iggy::identifier::Identifier;
use message_producer::MessageProducer;
use message_shared::{IggyConfig, IggyUser};
use std::fmt::Error;

pub struct MessageStream {
    client_id: u16,
    stream_id: Identifier,
    topic_id: Identifier,
    iggy_client: IggyClient,
    iggy_producer: MessageProducer,
}

impl MessageStream {
    /// Creates a new `MessageStream` instance with debugging disabled.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The client ID to use for this instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the newly created `MessageStream` instance or an error.
    ///
    pub async fn new(client_id: u16) -> Result<Self, Error> {
        Self::build(false, client_id).await
    }

    /// Creates a new `MessageStream` instance with debugging enabled.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The client ID to use for this instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the newly created `MessageStream` instance or an error.
    ///
    pub async fn with_debug(client_id: u16) -> Result<Self, Error> {
        Self::build(true, client_id).await
    }

    async fn build(dbg: bool, client_id: u16) -> Result<Self, Error> {
        let user = IggyUser::default();
        let iggy_config = IggyConfig::from_client_id(&user, client_id);
        let stream_id = iggy_config.stream_id();
        let topic_id = iggy_config.topic_id();

        let stream_name = iggy_config.stream_name().to_string();
        let topic_name = iggy_config.topic_name().to_string();

        let iggy_client = message_shared::build_client(&iggy_config)
            .await
            .expect("Failed to build client");

        iggy_client.connect().await.expect("Failed to connect");

        iggy_client
            .login_user(iggy_config.user().username(), iggy_config.user().password())
            .await
            .expect("Failed to login user");

        let iggy_producer = MessageProducer::from_client(
            dbg,
            &iggy_client,
            stream_name.clone(),
            topic_name.clone(),
        )
        .await
        .expect("Failed to create producer");

        Ok(Self {
            client_id,
            stream_id,
            topic_id,
            iggy_client,
            iggy_producer,
        })
    }
}

impl MessageStream {
    pub fn client_id(&self) -> u16 {
        self.client_id
    }

    pub fn stream_id(&self) -> &Identifier {
        &self.stream_id
    }

    pub fn topic_id(&self) -> &Identifier {
        &self.topic_id
    }

    pub fn iggy_client(&self) -> &IggyClient {
        &self.iggy_client
    }

    pub fn iggy_producer(&self) -> &MessageProducer {
        &self.iggy_producer
    }
}
