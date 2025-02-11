use crate::builder::IggyStreamConfig;
use iggy::client::Client;
use iggy::clients::client::IggyClient;
use iggy::clients::consumer::IggyConsumer;
use iggy::clients::producer::IggyProducer;
use iggy::error::IggyError;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IggyStream {}

impl IggyStream {
    pub async fn new(
        client: &IggyClient,
        config: &IggyStreamConfig,
    ) -> Result<(IggyProducer, IggyConsumer), IggyError> {
        // Build iggy producer
        let iggy_producer = match Self::build_iggy_producer(client, config).await {
            Ok(iggy_producer) => iggy_producer,
            Err(err) => return Err(err),
        };

        // Build iggy consumer
        let iggy_consumer = match Self::build_iggy_consumer(client, config).await {
            Ok(iggy_consumer) => iggy_consumer,
            Err(err) => return Err(err),
        };

        Ok((iggy_producer, iggy_consumer))
    }

    pub async fn with_client_from_connection_string(
        connection_string: &str,
        config: &IggyStreamConfig,
    ) -> Result<(IggyClient, IggyProducer, IggyConsumer), IggyError> {
        // Build and connect iggy client
        let client = IggyClient::from_connection_string(connection_string)?;
        client.connect().await?;

        // Build iggy producer and consumer
        let (iggy_producer, iggy_consumer) = Self::new(&client, config).await?;
        Ok((client, iggy_producer, iggy_consumer))
    }
}
