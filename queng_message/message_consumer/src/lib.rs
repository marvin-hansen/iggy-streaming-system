use futures_util::{Stream, StreamExt};
use iggy::clients::client::IggyClient;
use iggy::clients::consumer::{AutoCommit, AutoCommitWhen, IggyConsumer};
use iggy::consumer::ConsumerKind;
use iggy::error::IggyError;
use iggy::identifier::Identifier;
use iggy::messages::poll_messages::PollingStrategy;
use iggy::utils::duration::IggyDuration;
use message_shared::Args;
use std::str::FromStr;

mod event_consumer;
mod event_error_handler;
mod getters;
mod shutdown;

type Guarded<T> = std::sync::Arc<tokio::sync::RwLock<T>>;

pub struct MessageConsumer {
    dbg: bool,
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
        dbg: bool,
        client: &IggyClient,
        consumer_name: &str,
        stream_id: String,
        topic_id: String,
    ) -> Result<Self, IggyError> {
        let args = Args::new(stream_id, topic_id);
        Self::build(dbg, args, client, consumer_name).await
    }
}

impl MessageConsumer {
    async fn build(
        dbg: bool,
        args: Args,
        client: &IggyClient,
        consumer_name: &str,
    ) -> Result<Self, IggyError> {
        let stream_id = Identifier::from_str_value(&args.stream_id)
            .expect("[MessageConsumer]: Invalid stream id");
        let topic_id = Identifier::from_str_value(&args.topic_id).expect("Invalid topic id");

        let mut consumer = match ConsumerKind::from_code(args.consumer_kind)
            .expect("[MessageConsumer]: Invalid consumer kind")
        {
            ConsumerKind::Consumer => client
                .consumer(
                    consumer_name,
                    &args.stream_id,
                    &args.topic_id,
                    args.partition_id,
                )
                .expect("[MessageConsumer]: Failed to create consumer"),
            ConsumerKind::ConsumerGroup => client
                .consumer_group(consumer_name, &args.stream_id, &args.topic_id)
                .expect("[MessageConsumer]: Failed to create consumer group"),
        }
        .auto_commit(AutoCommit::When(AutoCommitWhen::PollingMessages))
        .create_consumer_group_if_not_exists()
        .auto_join_consumer_group()
        .polling_strategy(PollingStrategy::next())
        .poll_interval(
            IggyDuration::from_str(&args.interval)
                .expect("[MessageConsumer]: Invalid interval format"),
        )
        .batch_size(args.messages_per_batch)
        .build();

        consumer
            .init()
            .await
            .expect("[MessageConsumer]: Failed to initialize consumer");

        Ok(Self {
            dbg,
            consumer,
            stream_id,
            topic_id,
        })
    }
}

impl MessageConsumer {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[MessageConsumer]: {msg}");
        }
    }
}
