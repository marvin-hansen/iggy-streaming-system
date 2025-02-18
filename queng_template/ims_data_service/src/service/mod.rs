mod run;
mod shutdown;

use common_exchange::ExchangeID;
use common_ims::IntegrationConfig;
use iggy::clients::client::IggyClient;
use iggy::clients::consumer::IggyConsumer;
use iggy::clients::producer::IggyProducer;
use iggy::stream_builder::{IggyProducerConfig, IggyStream, IggyStreamConfig};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use trait_data_integration::ImsDataIntegration;

type Guarded<T> = Arc<tokio::sync::RwLock<T>>;

/// A server that handles IMS (Integration Management Service) data processing.
///
/// The server manages message consumption and production for both control and data channels,
/// maintaining thread-safe access to shared resources using Tokio's async-aware locks.
pub struct Service<Integration: ImsDataIntegration> {
    dbg: bool,
    exchange_id: ExchangeID,
    consumer: Guarded<IggyConsumer>,
    producer: Guarded<IggyProducer>,
    ims_integration: Guarded<Integration>,
    iggy_client: Guarded<IggyClient>,
    client_configs: Guarded<HashMap<u16, IggyProducerConfig>>,
    client_producers: Guarded<HashMap<u16, Arc<IggyProducer>>>,
}

impl<Integration: ImsDataIntegration> Service<Integration> {
    pub async fn build_service(
        dbg: bool,
        iggy_client: Guarded<IggyClient>,
        ims_integration: Integration,
        integration_config: &IntegrationConfig,
        iggy_config: &IggyStreamConfig,
    ) -> Result<Self, Box<dyn Error>> {
        Self::build(
            dbg,
            iggy_client,
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
        iggy_client: Guarded<IggyClient>,
        ims_integration: Integration,
        integration_config: &IntegrationConfig,
        iggy_stream_config: &IggyStreamConfig,
    ) -> Result<Self, Box<dyn Error>> {
        let dbg_print = |msg: &str| {
            if dbg {
                println!("[/Service]: {msg}");
            }
        };

        let exchange_id = integration_config.exchange_id();
        dbg_print("Create MessageProducer");
        let client_guard = iggy_client.write().await;
        let (producer, consumer) = IggyStream::build(&client_guard, iggy_stream_config)
            .await
            .expect("Failed to create producer");
        drop(client_guard);

        let consumer = Arc::new(tokio::sync::RwLock::new(consumer));
        let producer = Arc::new(tokio::sync::RwLock::new(producer));
        dbg_print("producer and consumer created");

        dbg_print("Create HashMaps");
        let client_configs = Arc::new(tokio::sync::RwLock::new(HashMap::new()));
        let client_producers = Arc::new(tokio::sync::RwLock::new(HashMap::new()));
        let ims_integration = Arc::new(tokio::sync::RwLock::new(ims_integration));

        dbg_print("Create Service");
        Ok(Self {
            dbg,
            exchange_id,
            consumer,
            producer,
            ims_integration,
            iggy_client,
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

    pub fn iggy_client(&self) -> &Guarded<IggyClient> {
        &self.iggy_client
    }

    pub fn client_configs(&self) -> &Guarded<HashMap<u16, IggyProducerConfig>> {
        &self.client_configs
    }
}

impl<Integration: ImsDataIntegration> Service<Integration> {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[IMSData/Server]: {msg}");
        }
    }
}
