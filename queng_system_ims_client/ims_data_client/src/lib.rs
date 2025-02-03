mod client_api;
mod client_mock;
mod client_trait;
mod error;
mod handler;
mod shutdown;

// Re-export the trait and error type
pub use client_trait::ImsDataClientTrait;
use common_data_bar::TimeResolution;
use common_exchange::ExchangeID;
use common_ims::IntegrationConfig;
use enum_dispatch::enum_dispatch;
pub use error::ImsClientError;
use iggy::client::{Client, UserClient};
use iggy::clients::client::IggyClient;
use message_consumer::MessageConsumer;
use message_producer::MessageProducer;
use message_shared::{IggyConfig, IggyUser};
use tokio::task::JoinHandle;
use trait_event_consumer::EventConsumer;

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
    exchange_id: ExchangeID,
    integration_config: IntegrationConfig,
    iggy_client_control: IggyClient,
    iggy_client_data: IggyClient,
    control_producer: MessageProducer,
    data_producer: MessageProducer,
    handler_control_consumer: JoinHandle<()>,
    handler_data_consumer: JoinHandle<()>,
}

impl ImsDataClient {
    pub async fn new(
        client_id: u16,
        integration_config: IntegrationConfig,
        control_event_processor: &'static (impl EventConsumer + Sync),
        data_event_processor: &'static (impl EventConsumer + Sync),
    ) -> Result<Self, ImsClientError> {
        Self::build(
            false,
            client_id,
            integration_config,
            control_event_processor,
            data_event_processor,
        )
            .await
    }

    pub async fn with_debug(
        client_id: u16,
        integration_config: IntegrationConfig,
        control_event_processor: &'static (impl EventConsumer + Sync),
        data_event_processor: &'static (impl EventConsumer + Sync),
    ) -> Result<Self, ImsClientError> {
        Self::build(
            true,
            client_id,
            integration_config,
            control_event_processor,
            data_event_processor,
        )
            .await
    }

    pub async fn build(
        dbg: bool,
        client_id: u16,
        integration_config: IntegrationConfig,
        control_event_processor: &'static (impl EventConsumer + Sync),
        data_event_processor: &'static (impl EventConsumer + Sync),
    ) -> Result<Self, ImsClientError> {
        let exchange_id = integration_config.exchange_id();

        // ###############################################################################
        // # Control stream: Build iggy client
        // ###############################################################################
        let control_stream_id = integration_config.control_channel();
        let control_topic_id = integration_config.control_channel();

        if dbg {
            println!("[ImsDataClient]: control_stream_id: {control_stream_id}");
            println!("[ImsDataClient]: control_topic_id: {control_topic_id}");
        }

        let iggy_config = IggyConfig::from_client_id(&IggyUser::default(), client_id);
        let iggy_client_control =
            match message_shared::build_client(control_stream_id.clone(), control_topic_id.clone())
                .await
            {
                Ok(client) => client,
                Err(err) => return Err(ImsClientError::FailedToCreateIggyClient(err.to_string())),
            };

        match iggy_client_control.connect().await {
            Ok(_) => {}
            Err(err) => {
                return Err(ImsClientError::FailedToConnectToIggyServer(format!(
                    "[ImsDataClient]: Failed to connect to control topic: {err}"
                )))
            }
        };

        let username = iggy_config.user().username();
        let password = iggy_config.user().password();
        match iggy_client_control.login_user(username, password).await {
            Ok(_) => {}
            Err(err) => {
                return Err(ImsClientError::FailedToLoginIggyUser(format!(
                    "[ImsDataClient]: Failed to login user {} due to error: {}",
                    username, err
                )))
            }
        };

        // ###############################################################################
        // # Control stream: Build producer
        // ###############################################################################
        let control_producer = match MessageProducer::from_client(
            dbg,
            &iggy_client_control,
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

        // ###############################################################################
        // # Control stream: Build consumer
        // ###############################################################################
        let consumer_name = "control_producer";
        let control_consumer = match MessageConsumer::from_client(
            &iggy_client_control,
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

        let handler_control_consumer = tokio::spawn(async move {
            match control_consumer
                .consume_messages(control_event_processor)
                .await
            {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("[ImsDataClient]: Failed to consume control messages: {err}");
                }
            }
        });

        // ###############################################################################
        // # Data stream: Build iggy client
        // ###############################################################################
        let data_stream_id = integration_config.data_channel();
        let data_topic_id = integration_config.data_channel();

        if dbg {
            println!("[ImsDataClient]: data_stream_id: {data_stream_id}");
            println!("[ImsDataClient]: data_topic_id: {data_topic_id}");
        }

        let iggy_config = IggyConfig::from_client_id(&IggyUser::default(), client_id);
        let iggy_client_data =
            match message_shared::build_client(data_stream_id.clone(), data_topic_id.clone()).await
            {
                Ok(client) => client,
                Err(err) => return Err(ImsClientError::FailedToCreateIggyClient(err.to_string())),
            };

        match iggy_client_data.connect().await {
            Ok(_) => {}
            Err(err) => {
                return Err(ImsClientError::FailedToConnectToIggyServer(format!(
                    "[ImsDataClient]: Failed to connect to control topic: {err}"
                )))
            }
        };

        let username = iggy_config.user().username();
        let password = iggy_config.user().password();
        match iggy_client_data.login_user(username, password).await {
            Ok(_) => {}
            Err(err) => {
                return Err(ImsClientError::FailedToLoginIggyUser(format!(
                    "[ImsDataClient]: Failed to login user {} due to error: {}",
                    username, err
                )))
            }
        };

        // ###############################################################################
        // # Data stream: Build Producer
        // ###############################################################################
        let data_producer = match MessageProducer::from_client(
            dbg,
            &iggy_client_control,
            data_stream_id.clone(),
            data_topic_id.clone(),
        )
            .await
        {
            Ok(producer) => producer,
            Err(err) => {
                return Err(ImsClientError::FailedToCreateIggyProducer(format!(
                    "[ImsDataClient]: Failed to create data channel producer: {err}"
                )))
            }
        };

        // ###############################################################################
        // # Data stream: Build consumer
        // ###############################################################################
        let consumer_name = "data_consumer";
        let data_consumer = match MessageConsumer::from_client(
            &iggy_client_data,
            consumer_name,
            data_stream_id.clone(),
            data_topic_id.clone(),
        )
            .await
        {
            Ok(consumer) => consumer,
            Err(err) => {
                return Err(ImsClientError::FailedToCreateIggyConsumer(format!(
                    "[ImsDataClient]: Failed to create data channel consumer: {err}"
                )))
            }
        };

        let handler_data_consumer = tokio::spawn(async move {
            match data_consumer.consume_messages(data_event_processor).await {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("[ImsDataClient]: Failed to consume data messages: {err}");
                }
            }
        });

        Ok(Self {
            dbg,
            client_id,
            exchange_id,
            integration_config,
            iggy_client_control,
            iggy_client_data,
            control_producer,
            data_producer,
            handler_control_consumer,
            handler_data_consumer,
        })
    }
}

impl ImsDataClient {
    pub fn client_id(&self) -> u16 {
        self.client_id
    }

    pub fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }

    pub fn integration_config(&self) -> &IntegrationConfig {
        &self.integration_config
    }

    pub fn control_producer(&self) -> &MessageProducer {
        &self.control_producer
    }

    pub fn data_producer(&self) -> &MessageProducer {
        &self.data_producer
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
        _control_event_processor: &'static (impl EventConsumer + Sync),
        _data_event_processor: &'static (impl EventConsumer + Sync),
    ) -> Result<Self, ImsClientError> {
        Ok(Self {
            time_resolution: TimeResolution::NoValue,
        })
    }
}
