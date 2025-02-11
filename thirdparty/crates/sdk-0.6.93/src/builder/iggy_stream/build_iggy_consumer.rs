use crate::builder::iggy_stream::IggyStream;
use crate::builder::IggyStreamConfig;
use iggy::clients::client::IggyClient;
use iggy::clients::consumer::{AutoCommit, AutoCommitWhen, IggyConsumer};
use iggy::consumer::ConsumerKind;
use iggy::error::IggyError;
use tracing::error;

impl IggyStream {
    /// Builds an `IggyConsumer` from the given `IggyClient` and `IggyStreamConfig`.
    ///
    /// # Arguments
    ///
    /// * `client` - The `IggyClient` to use.
    /// * `stream_config` - The `IggyStreamConfig` to use.
    ///
    /// # Errors
    ///
    /// * `IggyError` - If the iggy consumer cannot be build.
    ///
    /// # Details
    ///
    /// This function will create a new `IggyConsumer` with the given `IggyClient` and `IggyStreamConfig`.
    /// The `IggyStreamConfig` fields are used to configure the `IggyConsumer`.
    ///
    pub(crate) async fn build_iggy_consumer(
        client: &IggyClient,
        stream_config: &IggyStreamConfig,
    ) -> Result<IggyConsumer, IggyError> {
        // Extract config fields.
        let consumer_kind = stream_config.consumer_kind();
        let consumer_name = stream_config.consumer_group_name();
        let stream = stream_config.stream_name();
        let topic = stream_config.topic_name();
        let batch_size = stream_config.batch_size();
        let polling_interval = stream_config.polling_interval();
        let polling_strategy = stream_config.polling_strategy();

        // Build consumer.
        let mut consumer = match consumer_kind {
            ConsumerKind::Consumer => client.consumer(consumer_name, stream, topic, 1)?,
            ConsumerKind::ConsumerGroup => client.consumer_group(consumer_name, stream, topic)?,
        }
        .auto_commit(AutoCommit::When(AutoCommitWhen::PollingMessages))
        .create_consumer_group_if_not_exists()
        .auto_join_consumer_group()
        .polling_strategy(polling_strategy)
        .poll_interval(polling_interval)
        .batch_size(batch_size)
        .build();

        match consumer.init().await {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to initialize consumer: {}", err);
                return Err(err);
            }
        }

        Ok(consumer)
    }
}
