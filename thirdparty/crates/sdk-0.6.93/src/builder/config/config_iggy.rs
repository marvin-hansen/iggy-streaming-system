use std::fmt::{Display, Formatter};

use crate::builder::config::{IggyTcpTLSConfig, IggyUser};
use iggy::identifier::Identifier;

/// Configuration for Iggy, containing user information, stream and topic identifiers,
/// server address, partition details, and message handling settings.
#[derive(Debug, PartialEq, Clone)]
pub struct IggyConfig {
    user: IggyUser,
    stream_id: Identifier,
    stream_name: String,
    stream_partition_count: u32,
    topic_id: Identifier,
    topic_name: String,
    tcp_server_addr: Option<String>,
    tcp_tls_config: Option<IggyTcpTLSConfig>,
    partition_id: u32,
    message_consumer_name: String,
    messages_per_batch: u32,
    auto_commit: bool,
}

/// IggyConfig provides a configuration for Iggy, containing user information, stream and topic
/// identifiers, server address, partition details, and message handling settings.
///
/// The IggyConfig can be created with the `new` method with all the required parameters, or
/// with the `from_client_id` method which auto-generates the stream, topic and partition id
/// from the client id.
///
/// The IggyConfig can be created with the `new` method with all the required parameters, or
/// with the `from_client_id` method which auto-generates the stream, topic and partition id
/// from the client id.
///
/// The `from_client_id` method is mainly used for testing purposes.
impl IggyConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user: IggyUser,
        stream_id: u32,
        stream_name: String,
        stream_partition_count: u32,
        topic_id: u32,
        topic_name: String,
        tcp_server_addr: Option<String>,
        tcp_tls_config: Option<IggyTcpTLSConfig>,
        partition_id: u32,
        message_consumer_name: String,
        messages_per_batch: u32,
        auto_commit: bool,
    ) -> Self {
        Self {
            user,
            stream_id: Identifier::numeric(stream_id).unwrap(),
            stream_name,
            stream_partition_count,
            topic_id: Identifier::numeric(topic_id).unwrap(),
            topic_name,
            tcp_server_addr,
            tcp_tls_config,
            partition_id,
            message_consumer_name,
            messages_per_batch,
            auto_commit,
        }
    }
}

impl IggyConfig {
    /// Returns a copy of the `stream_id`
    pub fn stream_id(&self) -> Identifier {
        self.stream_id.to_owned()
    }

    /// Returns a reference to the `stream_name`
    pub fn stream_name(&self) -> &str {
        &self.stream_name
    }

    /// Returns the `stream_partition_count`
    pub fn stream_partition_count(&self) -> u32 {
        self.stream_partition_count
    }

    /// Returns a copy of the `topic_id`
    pub fn topic_id(&self) -> Identifier {
        self.topic_id.to_owned()
    }

    /// Returns a reference to the `topic_name`
    pub fn topic_name(&self) -> &str {
        &self.topic_name
    }

    /// Returns the `partition_id`
    pub fn partition_id(&self) -> u32 {
        self.partition_id
    }

    /// Returns the `messages_per_batch`
    pub fn messages_per_batch(&self) -> u32 {
        self.messages_per_batch
    }

    /// Returns the `auto_commit` flag
    pub fn auto_commit(&self) -> bool {
        self.auto_commit
    }

    /// Returns a copy of the `tcp_server_addr`
    pub fn tcp_server_addr(&self) -> Option<String> {
        self.tcp_server_addr.to_owned()
    }

    /// Returns a reference to the `tcp_tls_config`
    pub fn tcp_tls_config(&self) -> Option<IggyTcpTLSConfig> {
        self.tcp_tls_config.to_owned()
    }

    /// Returns a reference to the `user`
    pub fn user(&self) -> &IggyUser {
        &self.user
    }

    /// Returns a reference to the `message_consumer_name`
    pub fn message_consumer_name(&self) -> &str {
        &self.message_consumer_name
    }
}

impl Default for IggyConfig {
    fn default() -> Self {
        Self {
            user: IggyUser::default(),
            stream_id: Identifier::numeric(1).unwrap(),
            stream_name: "default_stream".to_string(),
            stream_partition_count: 1,
            topic_id: Identifier::numeric(1).unwrap(),
            topic_name: "default_topic".to_string(),
            partition_id: 1,
            messages_per_batch: 1,
            auto_commit: true,
            tcp_server_addr: Some("localhost:8090".to_string()),
            tcp_tls_config: None,
            message_consumer_name: "default_consumer".to_string(),
        }
    }
}

impl Display for IggyConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IggyConfig {{ user: {}, stream_id: {}, stream_name: {}, stream_partition_count: {}, \
             topic_id: {}, topic_name: {}, partition_id: {}, messages_per_batch: {}, auto_commit: {}, \
             tcp_server_addr: {:?}, tcp_tls_config: {:?}, message_consumer_name: {} }}",
            self.user, self.stream_id, self.stream_name, self.stream_partition_count,
            self.topic_id, self.topic_name, self.partition_id, self.messages_per_batch,
            self.auto_commit, self.tcp_server_addr, self.tcp_tls_config,
            self.message_consumer_name
        )
    }
}
