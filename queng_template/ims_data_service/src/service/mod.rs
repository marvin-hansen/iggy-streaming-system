mod run;
mod shutdown;

use common_exchange::ExchangeID;
use common_ims::IntegrationConfig;
use iggy::clients::client::IggyClient;
use iggy::clients::consumer::IggyConsumer;
use iggy::clients::producer::IggyProducer;
use iggy::stream_builder::IggyStream;
use iggy::stream_config::IggyStreamConfig;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use trait_data_integration::ImsDataIntegration;

type Guarded<T> = std::sync::Arc<tokio::sync::RwLock<T>>;

/// A server that handles IMS (Integration Management Service) data processing.
///
/// The server manages message consumption and production for both control and data channels,
/// maintaining thread-safe access to shared resources using Tokio's async-aware locks.
#[allow(dead_code)] // Supress dead code warning until its clear which fields to remove.
pub struct Service<Integration: ImsDataIntegration> {
    dbg: bool,
    exchange_id: ExchangeID,
    consumer: Guarded<IggyConsumer>,
    producer: Guarded<IggyProducer>,
    iggy_config: IggyStreamConfig,
    ims_integration: Guarded<Integration>,
    integration_config: IntegrationConfig,
    client_configs: Guarded<HashMap<u16, IggyStreamConfig>>,
    client_producers: Guarded<HashMap<u16, Arc<IggyProducer>>>,
}

impl<Integration: ImsDataIntegration> Service<Integration> {
    /// Creates a new instance of the service.
    ///
    /// # Arguments
    ///
    /// * `dbg` - A boolean flag to enable debug printing.
    /// * `consumer_client` - The `IggyClient` instance used for consuming messages.
    /// * `producer_client` - The `IggyClient` instance used for producing messages.
    /// * `ims_integration` - The integration instance to use for IMS data processing.
    /// * `integration_config` - The configuration for the integration.
    /// * `iggy_config` - The configuration for the Iggy client.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `Service` instance or an `Error`.
    ///
    pub async fn build_service(
        dbg: bool,
        consumer_client: &IggyClient,
        producer_client: &IggyClient,
        ims_integration: Integration,
        integration_config: &IntegrationConfig,
        iggy_config: &IggyStreamConfig,
    ) -> Result<Self, Box<dyn Error>> {
        Self::build(
            dbg,
            consumer_client,
            producer_client,
            ims_integration,
            integration_config,
            iggy_config,
        )
        .await
    }
}

impl<Integration: ImsDataIntegration> Service<Integration> {
    async fn build(
        dbg: bool,
        _consumer_client: &IggyClient,
        producer_client: &IggyClient,
        ims_integration: Integration,
        integration_config: &IntegrationConfig,
        iggy_config: &IggyStreamConfig,
    ) -> Result<Self, Box<dyn Error>> {
        let dbg_print = |msg: &str| {
            if dbg {
                println!("[/Service]: {msg}");
            }
        };

        let exchange_id = integration_config.exchange_id();

        dbg_print("Create Identifiers for control stream and topic");
        let stream_id = integration_config.control_channel();
        let topic_id = integration_config.control_channel();

        dbg_print(&format!("stream_id: {stream_id}"));
        dbg_print(&format!("topic_id: {topic_id}"));

        dbg_print("Create MessageProducer");
        let (producer, consumer) = IggyStream::build(producer_client, iggy_config)
            .await
            .expect("Failed to create producer");

        let consumer = std::sync::Arc::new(tokio::sync::RwLock::new(consumer));
        let producer = std::sync::Arc::new(tokio::sync::RwLock::new(producer));
        dbg_print("producer and consumer created");

        // Create a new HashMap to store data producers for each client
        dbg_print("Create HashMaps");
        let client_configs = std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new()));
        let client_producers = std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new()));
        let ims_integration = std::sync::Arc::new(tokio::sync::RwLock::new(ims_integration));

        dbg_print("Create Service");
        Ok(Self {
            dbg,
            exchange_id,
            consumer,
            producer,
            iggy_config: iggy_config.clone(),
            ims_integration,
            integration_config: integration_config.clone(),
            client_configs,
            client_producers,
        })
    }
}

impl<Integration: ImsDataIntegration> Service<Integration> {
    pub fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }

    pub fn ims_integration(&self) -> &Guarded<Integration> {
        &self.ims_integration
    }

    pub fn client_producers(&self) -> &Guarded<HashMap<u16, Arc<IggyProducer>>> {
        &self.client_producers
    }

    pub fn consumer(&self) -> &Guarded<IggyConsumer> {
        &self.consumer
    }

    pub fn producer(&self) -> &Guarded<IggyProducer> {
        &self.producer
    }
}

impl<Integration: ImsDataIntegration> Service<Integration> {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[IMSData/Server]: {msg}");
        }
    }
}
