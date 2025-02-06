use crate::builder::event_consumer::EventConsumer;
use crate::builder::message_consumer::MessageConsumer;
use futures_util::stream::StreamExt;
use iggy::error::IggyError;
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::error;

impl MessageConsumer {
    /// Consume messages from the underlying consumer and process them
    /// using the provided `EventConsumer` implementation.
    ///
    /// # Arguments
    ///
    /// * `event_processor` - A shared reference to the `EventConsumer` implementation to use for processing messages.
    /// * `cancellation_token` - A `CancellationToken` which can be used to cancel the message consumption loop.
    ///
    /// # Errors
    ///
    /// * `IggyError::Disconnected` - If the consumer is disconnected.
    /// * `IggyError::CannotEstablishConnection` - If the consumer cannot establish a connection.
    /// * `IggyError::StaleClient` - If the consumer is stale.
    /// * `IggyError::InvalidServerAddress` - If the server address is invalid.
    /// * `IggyError::InvalidClientAddress` - If the client address is invalid.
    /// * `IggyError::NotConnected` - If the consumer is not connected.
    /// * `IggyError::ClientShutdown` - If the consumer is shutdown.
    ///
    /// # Details
    ///
    /// This function will continue to consume messages from the underlying consumer until
    /// either the `cancellation_token` is canceled or an irrecoverable error occurs.
    ///
    /// If the `cancellation_token` is canceled, the function will return immediately.
    ///
    /// If an error occurs, the function will return the error.
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

                        match event_processor
                            .consume(message.message.payload.into())
                            .await{
                                Ok(()) => {}
                                Err(err) => {
                                    error!("Error while handling message: {err}");
                                }
                        };
                    }

                    Some(Err(err)) => {
                        match err {
                            IggyError::Disconnected => {
                                error!("Disconnected: shutdown client: {err}");
                                return Err(err);
                            }
                            IggyError::CannotEstablishConnection => {
                                error!("CannotEstablishConnection: shutdown client : {err}");
                                return Err(err);
                            }
                            IggyError::StaleClient => {
                                error!("StaleClient:  shutdown client: {err}");
                               return Err(err);
                            }
                            IggyError::InvalidServerAddress => {
                                error!("InvalidServerAddress:  shutdown client: {err}");
                              return Err(err);
                            }
                            IggyError::InvalidClientAddress => {
                                error!("InvalidClientAddress:  shutdown client: {err}");
                               return Err(err);
                            }
                            IggyError::NotConnected => {
                                error!("NotConnected:  shutdown client: {err}");
                                return Err(err);
                            }
                            IggyError::ClientShutdown => {
                                error!("ClientShutdown:  shutdown client: {err}");
                                return Err(err);
                            }
                            _ => {
                                error!("Error while handling message: {err}", );
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
