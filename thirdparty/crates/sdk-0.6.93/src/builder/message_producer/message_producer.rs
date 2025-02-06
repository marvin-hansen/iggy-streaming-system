
use crate::builder::config::Args;
use iggy::clients::client::IggyClient;
use iggy::clients::producer::IggyProducer;
use iggy::error::IggyError;
use iggy::identifier::Identifier;
use iggy::messages::send_messages::Partitioning;
use iggy::utils::duration::IggyDuration;
use iggy::utils::expiry::IggyExpiry;
use iggy::utils::topic_size::MaxTopicSize;
use std::str::FromStr;
use std::sync::Arc;
use tracing::error;

#[derive(Clone)]
pub struct MessageProducer {
    stream_id: Identifier,
    topic_id: Identifier,
    producer: Arc<IggyProducer>,
}

impl MessageProducer {
    /// Creates a new `MessageProducer` instance using the provided `IggyClient` and identifiers.
    ///
    /// # Arguments
    ///
    /// * `client` - The `IggyClient` to use for authentication and communication.
    /// * `stream_id` - The identifier of the stream.
    /// * `topic_id` - The identifier of the topic.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageProducer` instance or an `IggyError`.
    ///
    pub async fn from_client(
        client: &IggyClient,
        stream_id: String,
        topic_id: String,
    ) -> Result<Self, IggyError> {
        let args = Args::new(stream_id, topic_id);
        Self::build(args, client).await
    }
}

impl MessageProducer {
    async fn build(args: Args, client: &IggyClient) -> Result<Self, IggyError> {
        let stream_id = match Identifier::from_str_value(&args.stream_id) {
            Ok(id) => id,
            Err(err) => {
                error!(
                    "Failed to parse stream id for producer due to error: {}",
                    err
                );
                return Err(err);
            }
        };

        let topic_id = match Identifier::from_str_value(&args.topic_id) {
            Ok(id) => id,
            Err(err) => {
                error!(
                    "Failed to parse topic id for producer due to error: {}",
                    err
                );
                return Err(err);
            }
        };

        let send_interval = match IggyDuration::from_str(&args.interval) {
            Ok(interval) => interval,
            Err(err) => {
                error!(
                    "Failed to parse interval for producer due to error: {}",
                    err
                );
                // Add better error type
                return Err(IggyError::CommandLengthError(err.to_string()));
            }
        };

        let mut producer = client
            .producer(&args.stream_id, &args.topic_id)?
            .batch_size(args.messages_per_batch)
            .send_interval(send_interval)
            .partitioning(Partitioning::balanced())
            .create_topic_if_not_exists(
                args.partitions_count,
                None,
                IggyExpiry::ServerDefault,
                MaxTopicSize::ServerDefault,
            )
            .create_stream_if_not_exists()
            .build();

        match producer.init().await {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to initialize producer: {}", err);
                return Err(err);
            }
        };

        Ok(Self {
            stream_id,
            topic_id,
            producer: Arc::new(producer),
        })
    }
}

// Getters
impl MessageProducer {
    /// Returns a reference to the stream identifier.
    #[inline]
    pub const fn stream_id(&self) -> &Identifier {
        &self.stream_id
    }

    /// Returns a reference to the topic identifier.
    #[inline]
    pub const fn topic_id(&self) -> &Identifier {
        &self.topic_id
    }

    /// Returns a reference to the `IggyProducer`.
    #[inline]
    pub fn producer(&self) -> &IggyProducer {
        &self.producer
    }
}
