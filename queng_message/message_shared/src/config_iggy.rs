use std::fmt::{Display, Formatter};

use crate::config_iggy_user::IggyUser;
use iggy::identifier::Identifier;

/// Configuration for Iggy, containing user information, stream and topic identifiers,
/// server address, partition details, and message handling settings.
#[derive(Debug, PartialEq, Clone)]
pub struct IggyConfig {
    user: IggyUser,
    stream_id: Identifier,
    stream_name: String,
    topic_id: Identifier,
    topic_name: String,
    tcp_server_addr: String,
    partition_id: u32,
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
    pub fn new(
        user: IggyUser,
        tcp_server_addr: &str,
        stream_id: u32,
        topic_id: u32,
        partition_id: u32,
        messages_per_batch: u32,
        auto_commit: bool,
    ) -> Self {
        Self {
            user,
            stream_id: Identifier::numeric(stream_id).unwrap(),
            stream_name: format!("stream_{}", stream_id),
            topic_id: Identifier::numeric(topic_id).unwrap(),
            topic_name: format!("topic_{}", topic_id),
            tcp_server_addr: tcp_server_addr.to_owned(),
            partition_id,
            messages_per_batch,
            auto_commit,
        }
    }

    /// Creates an `IggyConfig` from a client id.
    ///
    /// `client_id` must be greater than 100.
    ///
    /// The `tcp_server_addr` is set to `"127.0.0.1:8090"`.
    ///
    /// The `messages_per_batch` is set to 10.
    ///
    /// The `auto_commit` is set to true.
    ///
    /// The `stream_id`, `stream_name`, `topic_id`, `topic_name`, and `partition_id` are all set to
    /// the value of `client_id`.
    pub fn from_client_id(user: &IggyUser, client_id: u16) -> Self {
        assert!(client_id >= 100, "id must be greater than 100");

        let client_id = client_id as u32;

        Self {
            user: user.to_owned(),
            stream_id: Identifier::numeric(client_id).unwrap(),
            stream_name: format!("stream_{}", client_id),
            topic_id: Identifier::numeric(client_id).unwrap(),
            topic_name: format!("topic_{}", client_id),
            tcp_server_addr: "127.0.0.1:8090".to_owned(),
            partition_id: client_id,
            messages_per_batch: 10,
            auto_commit: true,
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
    pub fn tcp_server_addr(&self) -> String {
        self.tcp_server_addr.to_owned()
    }

    /// Returns a reference to the `user`
    pub fn user(&self) -> &IggyUser {
        &self.user
    }
}

impl Display for IggyConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IggyConfig: \
            iggy_user: {} tcp_server_addr: {}, stream_id: {}, stream_name: {}, topic_id: {}, topic_name: {}, \
             partition_id: {}, messages_per_batch: {}, auto_commit: {}",
            self.user.username(),
            self.tcp_server_addr,
            self.stream_id,
            self.stream_name,
            self.topic_id,
            self.topic_name,
            self.partition_id,
            self.messages_per_batch,
            self.auto_commit,
        )
    }
}
