mod client_api;
mod client_mock;
mod client_trait;
mod config;
mod error;
mod handler;
mod shutdown;

use common_data_bar::TimeResolution;
use common_exchange::ExchangeID;
use common_ims::IntegrationConfig;
use enum_dispatch::enum_dispatch;
pub use error::ImsClientError;
use iggy::clients::client::IggyClient;
use message_client_builder::MessageClientBuilder;
use message_producer::MessageProducer;
use message_shared::{IggyConfig, IggyUser};
use tokio_util::sync::CancellationToken;
use trait_event_consumer::EventConsumer;

// Re-export pub use client_trait::ImsDataClientTrait;
pub use client_trait::ImsDataClientTrait;

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
    tx_control_consumer: CancellationToken,
    tx_data_consumer: CancellationToken,
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
}

impl ImsDataClient {
    pub async fn build(
        dbg: bool,
        client_id: u16,
        integration_config: IntegrationConfig,
        control_event_processor: &'static (impl EventConsumer + Sync),
        data_event_processor: &'static (impl EventConsumer + Sync),
    ) -> Result<Self, ImsClientError> {
        let exchange_id = integration_config.exchange_id();

        // ###############################################################################
        // # Control stream
        // ###############################################################################
        let iggy_control_stream_config = config::control_stream_config(exchange_id);
        let (iggy_client_control, control_builder) =
            MessageClientBuilder::build(dbg, &iggy_control_stream_config)
                .await
                .expect("Failed to build control stream");

        let control_producer = control_builder.iggy_producer().to_owned();
        let control_consumer = control_builder.iggy_consumer();

        // https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html#examples
        let token = CancellationToken::new();
        let tx_control_consumer = token.clone();
        tokio::spawn(async move {
            match control_consumer
                .consume_messages(control_event_processor, token)
                .await
            {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("[ImsDataClient]: Failed to consume control messages: {err}");
                }
            }
        });

        // ###############################################################################
        // # Data stream
        // ###############################################################################
        let iggy_data_stream_config = IggyConfig::from_client_id(&IggyUser::default(), client_id);
        let (iggy_client_data, data_builder) =
            MessageClientBuilder::build(dbg, &iggy_data_stream_config)
                .await
                .expect("Failed to build control stream");

        let data_producer = data_builder.iggy_producer().to_owned();
        let data_consumer = data_builder.iggy_consumer();

        // https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html#examples
        let token = CancellationToken::new();
        let tx_data_consumer = token.clone();
        tokio::spawn(async move {
            match data_consumer
                .consume_messages(data_event_processor, token)
                .await
            {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("[ImsDataClient]: Failed to start data consumer: {err}");
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
            tx_data_consumer,
            tx_control_consumer,
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
