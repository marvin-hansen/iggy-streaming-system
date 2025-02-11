mod client_api;
mod client_mock;
mod client_trait;
mod client_utils;
mod error;
mod handler;
mod shutdown;

use common_data_bar::TimeResolution;
use common_exchange::ExchangeID;
use common_ims::IntegrationConfig;
use enum_dispatch::enum_dispatch;
pub use error::ImsClientError;
use iggy::clients::client::IggyClient;
use iggy::clients::producer::IggyProducer;

use crate::client_utils::build_iggy_client;
// Re-export
pub use client_trait::ImsDataClientTrait;
pub use sdk::builder::{EventConsumer, EventConsumerError};
use sdk::builder::{IggyConsumerMessageExt, IggyStream};
use tokio::sync::oneshot;

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
    iggy_client_control: IggyClient,
    iggy_client_data: IggyClient,
    control_producer: IggyProducer,
    data_producer: IggyProducer,
    tx_control_consumer: tokio::sync::RwLock<Option<oneshot::Sender<()>>>,
    tx_data_consumer: tokio::sync::RwLock<Option<oneshot::Sender<()>>>,
}

impl ImsDataClient {
    pub async fn new(
        client_id: u16,
        exchange_id: ExchangeID,
        iggy_connection_string: &str,
        control_event_processor: &'static (impl EventConsumer + Sync),
        data_event_processor: &'static (impl EventConsumer + Sync),
    ) -> Result<Self, ImsClientError> {
        Self::build(
            false,
            client_id,
            exchange_id,
            iggy_connection_string,
            control_event_processor,
            data_event_processor,
        )
        .await
    }

    pub async fn with_debug(
        client_id: u16,
        exchange_id: ExchangeID,
        iggy_connection_string: &str,
        control_event_processor: &'static (impl EventConsumer + Sync),
        data_event_processor: &'static (impl EventConsumer + Sync),
    ) -> Result<Self, ImsClientError> {
        Self::build(
            true,
            client_id,
            exchange_id,
            iggy_connection_string,
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
        exchange_id: ExchangeID,
        iggy_connection_string: &str,
        control_event_processor: &'static (impl EventConsumer + Sync),
        data_event_processor: &'static (impl EventConsumer + Sync),
    ) -> Result<Self, ImsClientError> {
        // ###############################################################################
        // # Control stream
        // ###############################################################################
        let iggy_client_control = match build_iggy_client(iggy_connection_string).await {
            Ok(client) => client,
            Err(err) => {
                return Err(err);
            }
        };

        let iggy_control_stream_config = ims_iggy_config::ims_control_iggy_config(exchange_id);
        let (control_producer, control_consumer) =
            match IggyStream::new(&iggy_client_control, &iggy_control_stream_config).await {
                Ok(control_builder) => control_builder,
                Err(err) => {
                    return Err(ImsClientError::FailedToCreateIggyConsumer(err.to_string()));
                }
            };

        let (tx_control_sender, receiver) = oneshot::channel();
        tokio::spawn(async move {
            match control_consumer
                .consume_messages(control_event_processor, receiver)
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
        let iggy_client_data = match build_iggy_client(iggy_connection_string).await {
            Ok(client) => client,
            Err(err) => {
                return Err(err);
            }
        };

        let iggy_data_stream_config = ims_iggy_config::ims_data_iggy_config(client_id, exchange_id);
        let (data_producer, data_consumer) =
            match IggyStream::new(&iggy_client_data, &iggy_data_stream_config).await {
                Ok(data_builder) => data_builder,
                Err(err) => {
                    return Err(ImsClientError::FailedToCreateIggyConsumer(err.to_string()));
                }
            };

        let (tx_data_sender, receiver) = oneshot::channel();
        tokio::spawn(async move {
            match data_consumer
                .consume_messages(data_event_processor, receiver)
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
            iggy_client_control,
            iggy_client_data,
            control_producer,
            data_producer,
            tx_control_consumer: tokio::sync::RwLock::new(Some(tx_control_sender)),
            tx_data_consumer: tokio::sync::RwLock::new(Some(tx_data_sender)),
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

    pub fn control_producer(&self) -> &IggyProducer {
        &self.control_producer
    }

    pub fn data_producer(&self) -> &IggyProducer {
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
