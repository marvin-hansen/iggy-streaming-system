mod client_api;
mod client_mock;
mod client_trait;
mod error;
mod handler;
mod shutdown;
use common_data_bar::TimeResolution;
use common_exchange::ExchangeID;
use common_ims::IntegrationConfig;
use enum_dispatch::enum_dispatch;
use iggy::client::{Client, UserClient};
use iggy::clients::client::IggyClient;
use message_consumer::MessageConsumer;
use message_producer::MessageProducer;
use message_shared::{IggyConfig, IggyUser};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use trait_event_consumer::EventConsumer;

// Re-export the trait and error type
pub use client_trait::ImsDataClientTrait;
pub use error::ImsClientError;

/// The selector for the IMS data client allows
/// to select between the real and mock client while keeping the same client interface.
#[enum_dispatch]
pub enum ImsDataClientSelector {
    /// The real IMS data client
    ImsDataClient,
    /// The mock IMS data client
    ImsDataMockClient,
}

pub struct ImsDataClient {
    dbg: bool,
    client_id: u16,
    control_client: IggyClient,
    control_handler: JoinHandle<()>,
    control_producer: MessageProducer,
    exchange_id: ExchangeID,
}

impl ImsDataClient {
    pub async fn new(
        client_id: u16,
        integration_config: IntegrationConfig,
        data_event_processor: &'static (impl EventConsumer + Sync),
        shutdown_rx: oneshot::Receiver<()>,
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
        shutdown_rx: oneshot::Receiver<()>,
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
        let control_consumer = match MessageConsumer::from_client(
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

        let control_handler = tokio::spawn(async move {
            control_consumer
                .consume_messages(data_event_processor, shutdown_rx)
                .await;
        });

        Ok(Self {
            dbg,
            client_id,
            control_client,
            control_handler,
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

    pub fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }
}

impl ImsDataClient {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[ImsDataClient]: {msg}");
        }
    }
}

#[allow(dead_code)] // Ignore the unused field in the struct
#[derive(Debug, Clone)]
pub struct ImsDataMockClient {
    // This field is not used; however,  without it, the auto code formatter would
    // remove the TimeResolution import, which then causes the enum_dispatch macro to fail compilation.
    time_resolution: TimeResolution,
}

impl ImsDataMockClient {
    pub async fn new(
        _client_id: u16,
        _integration_config: IntegrationConfig,
        _data_event_processor: &'static (impl EventConsumer + Sync),
        _shutdown_rx: oneshot::Receiver<()>,
    ) -> Result<Self, ImsClientError> {
        Ok(Self {
            time_resolution: TimeResolution::NoValue,
        })
    }
}
