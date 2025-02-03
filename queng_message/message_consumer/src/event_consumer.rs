use crate::MessageConsumer;
use futures_util::stream::StreamExt;
use iggy::error::IggyError;
use trait_event_consumer::EventConsumer;

// https://discord.com/channels/1144142576266530928/1144142577369628684/1333360421842980866
impl MessageConsumer {
    pub async fn consume_messages(
        mut self,
        event_processor: &'static (impl EventConsumer + Sync),
    ) -> Result<(), IggyError> {
        tokio::spawn(async move {
            let consumer = &mut self.consumer;

            while let Some(received_message) = consumer.next().await {
                match received_message {
                    // Process received message
                    Ok(message) => event_processor
                        .consume(message.message.payload.into())
                        .await
                        .expect("[MessageConsumer]: Failed to process message"),

                    // Handle errors
                    Err(err) => match err {
                        IggyError::Disconnected => {
                            eprintln!("Disconnected:  shutdown client");
                            break;
                        }
                        IggyError::CannotEstablishConnection => {
                            eprintln!("CannotEstablishConnection:  shutdown client");
                            break;
                        }
                        IggyError::StaleClient => {
                            eprintln!("StaleClient:  shutdown client");
                            break;
                        }
                        IggyError::InvalidServerAddress => {
                            eprintln!("InvalidServerAddress:  shutdown client");
                            break;
                        }
                        IggyError::InvalidClientAddress => {
                            eprintln!("InvalidClientAddress:  shutdown client");
                            break;
                        }
                        IggyError::NotConnected => {
                            eprintln!("NotConnected:  shutdown client");
                            break;
                        }
                        IggyError::ClientShutdown => {
                            eprintln!("ClientShutdown:  shutdown client");
                            break;
                        }
                        _ => {
                            eprintln!("[MessageConsumer]: Error while handling message: {err}", );
                        }
                    },
                }
            }
        });

        Ok(())
    }
}
