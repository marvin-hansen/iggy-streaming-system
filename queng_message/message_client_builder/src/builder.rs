use crate::build_fields::ConfigFields;
use crate::{MessageClientBuilder, MessageClientBuilderError};
use iggy::client::{Client, UserClient};
use iggy::clients::client::IggyClient;
use message_consumer::MessageConsumer;
use message_producer::MessageProducer;
use message_shared::{Args, IggyConfig};

impl MessageClientBuilder {
    /// Builds a new `MessageClientBuilder` with the given iggy configuration and debug mode.
    ///
    /// Args:
    /// * `dbg`: A boolean flag to enable debug printing.
    /// * `iggy_config`: The configuration for the iggy client.
    ///
    /// Returns:
    /// A `Result` containing a tuple of:
    /// * A reference to the `IggyClient` created.
    /// * A reference to the `MessageClientBuilder` created.
    ///
    /// Errors:
    /// If the iggy client fails to build, a `MessageClientBuilderError` is returned.
    ///
    pub(crate) async fn build(
        dbg: bool,
        iggy_config: Option<&IggyConfig>,
        args: Option<(Args, String)>,
    ) -> Result<(IggyClient, Self), MessageClientBuilderError> {
        let dbg_print = |msg: &str| {
            if dbg {
                println!("[MessageClientBuilder]: {msg}");
            }
        };

        if iggy_config.is_none() && args.is_none() {
            return Err(MessageClientBuilderError::FailedToCreateIggyClient(
                "No iggy config or args provided".to_string(),
            ));
        }

        let config_fields = if iggy_config.is_some() {
            dbg_print("Build fields from iggy config");
            ConfigFields::from_iggy_config(iggy_config.unwrap())
        } else {
            dbg_print("Build fields from args");
            let (args, consumer_name) = args.clone().unwrap();
            ConfigFields::from_args(args, consumer_name)
        };

        let consumer_name = config_fields.consumer_name();
        let stream_id = config_fields.stream_id().to_string();
        let topic_id = config_fields.topic_id().to_string();
        dbg_print(&format!("consumer_name: {consumer_name}"));
        dbg_print(&format!("stream_id: {stream_id}"));
        dbg_print(&format!("topic_id: {topic_id}"));

        let username = config_fields.username();
        let password = config_fields.password();
        dbg_print(&format!("iggy username: {username}"));

        dbg_print("Build iggy client");

        let iggy_client = if let Some(iggy_config) = iggy_config {
            match message_shared::build_client(iggy_config).await {
                Ok(client) => client,
                Err(err) => {
                    return Err(MessageClientBuilderError::FailedToCreateIggyClient(
                        err.to_string(),
                    ))
                }
            }
        } else if let Some((args, _)) = args {
            match message_shared::build_client_from_args(args).await {
                Ok(client) => client,
                Err(err) => {
                    return Err(MessageClientBuilderError::FailedToCreateIggyClient(
                        err.to_string(),
                    ))
                }
            }
        } else {
            return Err(MessageClientBuilderError::FailedToCreateIggyClient(
                "Both iggy_config and args are not provided. Please provide a config to build iggy client.".to_string(),
            ));
        };

        dbg_print("Connect iggy client");
        match iggy_client.connect().await {
            Ok(_) => {}
            Err(err) => {
                return Err(MessageClientBuilderError::FailedToConnectToIggyServer(
                    format!(
                        "Failed to connect to control stream {} due to error : {}",
                        stream_id, err
                    ),
                ))
            }
        };

        dbg_print("Login iggy client");
        match iggy_client.login_user(username, password).await {
            Ok(_) => {}
            Err(err) => {
                return Err(MessageClientBuilderError::FailedToLoginIggyUser(format!(
                    "[ImsDataClient]: Failed to login user {} due to error: {}",
                    username, err
                )))
            }
        };

        let iggy_producer = match MessageProducer::from_client(
            dbg,
            &iggy_client,
            stream_id.clone(),
            topic_id.clone(),
        )
            .await
        {
            Ok(producer) => producer,
            Err(err) => {
                return Err(MessageClientBuilderError::FailedToCreateIggyProducer(
                    format!(
                        "Failed to create producer for stream: {} due to error {}",
                        stream_id, err
                    ),
                ))
            }
        };

        let iggy_consumer = match MessageConsumer::from_client(
            &iggy_client,
            consumer_name,
            stream_id.clone(),
            topic_id.clone(),
        )
            .await
        {
            Ok(consumer) => consumer,
            Err(err) => {
                return Err(MessageClientBuilderError::FailedToCreateIggyConsumer(
                    format!(
                        "Failed to create consumer for stream: {} due to error {}",
                        stream_id, err
                    ),
                ))
            }
        };

        Ok((
            iggy_client,
            Self {
                iggy_producer,
                iggy_consumer,
            },
        ))
    }
}
