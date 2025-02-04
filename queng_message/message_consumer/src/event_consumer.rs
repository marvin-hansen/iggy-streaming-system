use crate::MessageConsumer;
use futures_util::stream::StreamExt;
use iggy::error::IggyError;
use tokio::select;
use tokio_util::sync::CancellationToken;
use trait_event_consumer::EventConsumer;


// https://discord.com/channels/1144142576266530928/1144142577369628684/1333360421842980866
impl MessageConsumer {
    pub async fn consume_messages(
        mut self,
        event_processor: &'static (impl EventConsumer + Sync),
        cancellation_token: CancellationToken,
    ) -> Result<(), IggyError> {
        let consumer = &mut self.consumer;

        select! {
             _ = cancellation_token.cancelled() => {
                    return Ok(())
                }

            received_message = consumer.next() => {
                match received_message {

                    // Message received, process it
                    Some(Ok(message)) => {
                        event_processor
                            .consume(message.message.payload.into())
                            .await
                            .expect("[MessageConsumer]: Failed to process message");
                    }

                    Some(Err(err)) => {
                        match err {
                            IggyError::Disconnected => {
                                eprintln!("Disconnected:  shutdown client");
                                return Err(err);
                            }
                            IggyError::CannotEstablishConnection => {
                                eprintln!("CannotEstablishConnection:  shutdown client");
                                return Err(err);
                            }
                            IggyError::StaleClient => {
                                eprintln!("StaleClient:  shutdown client");
                               return Err(err);
                            }
                            IggyError::InvalidServerAddress => {
                                eprintln!("InvalidServerAddress:  shutdown client");
                              return Err(err);
                            }
                            IggyError::InvalidClientAddress => {
                                eprintln!("InvalidClientAddress:  shutdown client");
                               return Err(err);
                            }
                            IggyError::NotConnected => {
                                eprintln!("NotConnected:  shutdown client");
                                return Err(err);
                            }
                            IggyError::ClientShutdown => {
                                eprintln!("ClientShutdown:  shutdown client");
                                return Err(err);
                            }
                            _ => {
                                eprintln!("[MessageConsumer]: Error while handling message: {err}", );
                            }
                        } // end match error

                    } // end Some(error)

                    // No message  received, continue
                    None => {}

                } // end received_message

            }  // end polled messages

        } // end tokio select

        Ok(())
    }
}
