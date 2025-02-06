
use crate::builder::config::Args;
use iggy::clients::client::IggyClient;
use iggy::clients::consumer::{AutoCommit, AutoCommitWhen, IggyConsumer};
use iggy::consumer::ConsumerKind;
use iggy::error::IggyError;
use iggy::identifier::Identifier;
use iggy::messages::poll_messages::PollingStrategy;
use iggy::utils::duration::IggyDuration;
use std::str::FromStr;
use tracing::error;

pub struct MessageConsumer {
    consumer: IggyConsumer,
    stream_id: Identifier,
    topic_id: Identifier,
}

impl MessageConsumer {
    /// Creates a `MessageConsumer` instance using the provided `IggyClient` and configuration.
    ///
    /// # Arguments
    ///
    /// * `dbg` - A boolean flag to enable debug printing.
    /// * `client` - The `IggyClient` to use for creating the consumer.
    /// * `consumer_name` - The name of the consumer.
    /// * `stream_id` - The identifier of the stream.
    /// * `topic_id` - The identifier of the topic.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageConsumer` instance or an `IggyError`.
    ///
    pub async fn from_client(
        client: &IggyClient,
        consumer_name: &str,
        stream_id: String,
        topic_id: String,
    ) -> Result<Self, IggyError> {
        let args = Args::new(stream_id, topic_id);
        Self::build(args, client, consumer_name).await
    }
}

impl MessageConsumer {
    async fn build(
        args: Args,
        client: &IggyClient,
        consumer_name: &str,
    ) -> Result<Self, IggyError> {
        //
        let stream_id = match Identifier::from_str_value(&args.stream_id) {
            Ok(id) => id,
            Err(err) => {
                error!(
                    "Failed to parse stream id for consumer due to error: {}",
                    err
                );
                return Err(err);
            }
        };

        let topic_id = match Identifier::from_str_value(&args.topic_id) {
            Ok(id) => id,
            Err(err) => {
                error!(
                    "Failed to parse topic id for consumer due to error: {}",
                    err
                );
                return Err(err);
            }
        };

        let poll_interval = match IggyDuration::from_str(&args.interval) {
            Ok(interval) => interval,
            Err(err) => {
                error!(
                    "Failed to parse interval for consumer due to error: {}",
                    err
                );
                // Add better error type
                return Err(IggyError::CommandLengthError(err.to_string()));
            }
        };

        let mut consumer = match ConsumerKind::from_code(args.consumer_kind)? {
            ConsumerKind::Consumer => client.consumer(
                consumer_name,
                &args.stream_id,
                &args.topic_id,
                args.partition_id,
            )?,
            ConsumerKind::ConsumerGroup => {
                client.consumer_group(consumer_name, &args.stream_id, &args.topic_id)?
            }
        }
            .auto_commit(AutoCommit::When(AutoCommitWhen::PollingMessages))
            .create_consumer_group_if_not_exists()
            .auto_join_consumer_group()
            .polling_strategy(PollingStrategy::last())
            .poll_interval(poll_interval)
            .batch_size(args.messages_per_batch)
            .build();

        match consumer.init().await {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to initialize consumer: {}", err);
                return Err(err);
            }
        }

        Ok(Self {
            consumer,
            stream_id,
            topic_id,
        })
    }
}

// Getters
impl MessageConsumer {
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

    /// Returns a mutable reference to the underlying consumer.
    #[inline]
    pub fn consumer_mut(&mut self) -> &mut IggyConsumer {
        &mut self.consumer
    }
}
