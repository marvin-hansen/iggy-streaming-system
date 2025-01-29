mod error;

use crate::error::ImsClientError;
use common_exchange::ExchangeID;
use common_ims::IntegrationConfig;
use iggy::client::{Client, UserClient};
use iggy::clients::client::IggyClient;
use message_consumer::MessageConsumer;
use message_producer::MessageProducer;
use message_shared::{IggyConfig, IggyUser};

// type Guarded<T> = std::sync::Arc<tokio::sync::RwLock<T>>;

pub struct ImsDataClient {
    client_id: u16,
    control_client: IggyClient,
    control_consumer: MessageConsumer,
    control_producer: MessageProducer,
    exchange_id: ExchangeID,
}

impl ImsDataClient {
    pub async fn new(
        client_id: u16,
        integration_config: IntegrationConfig,
    ) -> Result<Self, ImsClientError> {
        Self::build(false, client_id, integration_config).await
    }

    pub async fn build(
        dbg: bool,
        client_id: u16,
        integration_config: IntegrationConfig,
    ) -> Result<Self, ImsClientError> {
        let exchange_id = integration_config.exchange_id();

        let control_stream_id = integration_config.control_channel();
        let control_topic_id = integration_config.control_channel();

        let user = IggyUser::default();
        let iggy_config = IggyConfig::from_client_id(user, client_id);
        let control_client =
            message_shared::build_client(control_stream_id.clone(), control_topic_id.clone())
                .await
                .expect("[ImsDataClient]: Failed to build client");

        control_client
            .connect()
            .await
            .expect("[ImsDataClient]: Failed to connect to iggy bus on control topic");

        control_client
            .login_user(iggy_config.user().username(), iggy_config.user().password())
            .await
            .expect("[ImsDataClient]: Failed to login user");

        let control_producer = MessageProducer::from_client(
            dbg,
            &control_client,
            control_stream_id.clone(),
            control_topic_id.clone(),
        )
            .await
            .expect("[ImsDataClient]: Failed to create producer");

        let control_consumer = MessageConsumer::from_client(
            dbg,
            &control_client,
            "control_consumer",
            control_stream_id.clone(),
            control_topic_id.clone(),
        )
            .await
            .expect("[ImsDataClient]:  Failed to create consumer");

        // control_consumer.consume_messages();

        Ok(Self {
            client_id,
            control_client,
            control_consumer,
            control_producer,
            exchange_id,
        })
    }
}

impl ImsDataClient {
    pub fn client_id(&self) -> u16 {
        self.client_id
    }

    pub fn control_client(&self) -> &IggyClient {
        &self.control_client
    }

    pub fn control_consumer(&self) -> &MessageConsumer {
        &self.control_consumer
    }

    pub fn control_producer(&self) -> &MessageProducer {
        &self.control_producer
    }

    pub fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }
}
