use crate::error::MessageClientBuilderError;
use iggy::client::Client;
use iggy::clients::client::IggyClient;
use message_consumer::MessageConsumer;
use message_producer::MessageProducer;
use message_shared::IggyConfig;

mod error;

pub struct MessageClientBuilder {
    iggy_config: IggyConfig,
    iggy_client: IggyClient,
    iggy_producer: MessageProducer,
    iggy_consumer: MessageConsumer,
}

impl MessageClientBuilder {
    pub async fn new(
        iggy_config: IggyConfig,
        consumer_name: &str,
    ) -> Result<Self, MessageClientBuilderError> {
        Self::build(false, iggy_config, consumer_name).await
    }

    pub async fn with_debug(
        iggy_config: IggyConfig,
        consumer_name: &str,
    ) -> Result<Self, MessageClientBuilderError> {
        Self::build(true, iggy_config, consumer_name).await
    }
}

impl MessageClientBuilder {
    async fn build(
        dbg: bool,
        iggy_config: IggyConfig,
        consumer_name: &str,
    ) -> Result<Self, MessageClientBuilderError> {
        let dbg_print = |msg: &str| {
            if dbg {
                println!("[MessageClientBuilder]: {msg}");
            }
        };

        let control_stream_id = iggy_config.stream_name().to_string();
        let control_topic_id = iggy_config.topic_name().to_string();
        dbg_print(&format!("control_stream_id: {control_stream_id}"));
        dbg_print(&format!("control_topic_id: {control_topic_id}"));

        dbg_print("Build iggy client");
        let iggy_client =
            match message_shared::build_client(control_stream_id.clone(), control_topic_id.clone())
                .await
            {
                Ok(client) => client,
                Err(err) => {
                    return Err(MessageClientBuilderError::FailedToCreateIggyClient(
                        err.to_string(),
                    ))
                }
            };

        dbg_print("Connect iggy client");
        match iggy_client.connect().await {
            Ok(_) => {}
            Err(err) => {
                return Err(MessageClientBuilderError::FailedToConnectToIggyServer(
                    format!(
                        "Failed to connect to control stream {} due to error : {}",
                        control_stream_id, err
                    ),
                ))
            }
        };

        let iggy_producer = match MessageProducer::from_client(
            dbg,
            &iggy_client,
            control_stream_id.clone(),
            control_topic_id.clone(),
        )
            .await
        {
            Ok(producer) => producer,
            Err(err) => {
                return Err(MessageClientBuilderError::FailedToCreateIggyProducer(
                    format!(
                        "Failed to create producer for stream: {} due to error {}",
                        control_stream_id, err
                    ),
                ))
            }
        };

        let iggy_consumer = match MessageConsumer::from_client(
            &iggy_client,
            consumer_name,
            control_stream_id.clone(),
            control_topic_id.clone(),
        )
            .await
        {
            Ok(consumer) => consumer,
            Err(err) => {
                return Err(MessageClientBuilderError::FailedToCreateIggyConsumer(
                    format!(
                        "Failed to create consumer for stream: {} due to error {}",
                        control_stream_id, err
                    ),
                ))
            }
        };

        Ok(Self {
            iggy_config,
            iggy_client,
            iggy_producer,
            iggy_consumer,
        })
    }
}

impl MessageClientBuilder {
    pub fn iggy_config(&self) -> &IggyConfig {
        &self.iggy_config
    }

    pub fn iggy_client(&self) -> &IggyClient {
        &self.iggy_client
    }

    pub fn iggy_producer(self) -> MessageProducer {
        self.iggy_producer
    }

    pub fn iggy_consumer(self) -> MessageConsumer {
        self.iggy_consumer
    }
}
