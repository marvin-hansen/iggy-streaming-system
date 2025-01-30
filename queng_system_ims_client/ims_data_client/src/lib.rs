mod error;

use crate::error::ImsClientError;
use common_exchange::ExchangeID;
use common_ims::IntegrationConfig;
use iggy::client::{Client, UserClient};
use iggy::clients::client::IggyClient;
use message_consumer::MessageConsumer;
use message_producer::MessageProducer;
use message_shared::{IggyConfig, IggyUser};
use tokio::sync::oneshot;
use trait_event_consumer::EventConsumer;

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
        data_event_processor: &'static (impl EventConsumer + Sync),
        shutdown_rx: oneshot::Receiver<()>, // or any `Future<Output=()>`
    ) -> Result<Self, ImsClientError> {
        Self::build(
            false,
            client_id,
            integration_config,
            data_event_processor,
            shutdown_rx,
        )
            .await
    }

    pub async fn build(
        dbg: bool,
        client_id: u16,
        integration_config: IntegrationConfig,
        data_event_processor: &'static (impl EventConsumer + Sync),
        shutdown_rx: oneshot::Receiver<()>, // or any `Future<Output=()>`
    ) -> Result<Self, ImsClientError> {
        let exchange_id = integration_config.exchange_id();

        let control_stream_id = integration_config.control_channel();
        let control_topic_id = integration_config.control_channel();

        let iggy_config = IggyConfig::from_client_id(&IggyUser::default(), client_id);
        let control_client =
            match message_shared::build_client(control_stream_id.clone(), control_topic_id.clone())
                .await
            {
                Ok(client) => client,
                Err(err) => return Err(ImsClientError::FailedToCreateIggyClient(err.to_string())),
            };

        match control_client.connect().await {
            Ok(_) => {}
            Err(err) => {
                return Err(ImsClientError::FailedToConnectToIggyServer(format!(
                    "[ImsDataClient]: Failed to connect to control topic: {err}"
                )))
            }
        };

        let username = iggy_config.user().username();
        let password = iggy_config.user().password();
        match control_client.login_user(username, password).await {
            Ok(_) => {}
            Err(err) => {
                return Err(ImsClientError::FailedToLoginIggyUser(format!(
                    "[ImsDataClient]: Failed to login user {} due to error: {}",
                    username, err
                )))
            }
        };

        let control_producer = match MessageProducer::from_client(
            dbg,
            &control_client,
            control_stream_id.clone(),
            control_topic_id.clone(),
        )
            .await
        {
            Ok(producer) => producer,
            Err(err) => {
                return Err(ImsClientError::FailedToCreateIggyProducer(format!(
                    "[ImsDataClient]: Failed to create control channel producer: {err}"
                )))
            }
        };

        let consumer_name = "control_producer";
        let mut control_consumer = match MessageConsumer::from_client(
            dbg,
            &control_client,
            consumer_name,
            control_stream_id.clone(),
            control_topic_id.clone(),
        )
            .await
        {
            Ok(consumer) => consumer,
            Err(err) => {
                return Err(ImsClientError::FailedToCreateIggyConsumer(format!(
                    "[ImsDataClient]: Failed to create control channel consumer: {err}"
                )))
            }
        };

        // @FIXME: Resolve cannot move error
        match control_consumer.consume_messages(data_event_processor, shutdown_rx) {
            Ok(_) => {}
            Err(err) => {
                return Err(ImsClientError::FailedToStartIggyConsumer(format!(
                    "[ImsDataClient]: Failed to start message consumer for control channel: {err}"
                )))
            }
        };

        Ok(Self {
            client_id,
            control_client,
            // Value used after being moved
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
