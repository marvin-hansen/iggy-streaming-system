use crate::service::Service;
use common_errors::MessageProcessingError;
use futures_util::StreamExt;
use std::future::Future;
use tokio::{pin, select};
use trait_data_integration::ImsDataIntegration;

impl<Integration> Service<Integration>
where
    Integration: ImsDataIntegration + Send + Sync + 'static,
{
    /// Starts the data service in an infinite loop.
    ///
    /// The service will consume messages from the control topic and handle them
    /// according to the service's configuration.
    ///
    /// # Errors
    ///
    /// The function will return an error if the service cannot be started.
    ///
    pub(crate) async fn run(
        self,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), MessageProcessingError> {
        self.dbg_print("Running integration service");
        // When call .await on a &mut _ reference, pin the future. https://docs.rs/tokio/latest/tokio/macro.pin.html#examples
        let signal_future = signal;
        pin!(signal_future);

        let consumer_guard = self.consumer().write().await;
        let mut consumer = consumer_guard;

        loop {
            select! {
            _ = &mut signal_future => {break;}

            received_message = consumer.next() => {
                    match received_message {

                        Some(Ok(message)) => {
                                    self.handle_message(message.message).await.expect("Failed to handle message");
                        },
                        Some(Err(e)) => {
                            eprintln!("[Service/run]: Error polling messages from iggy message bus: {}", e);
                            break;
                        }

                        None => {}
                    }
                } // end match polled messages
            } // end select
        } // end loop

        self.shutdown().await.expect("Failed to shutdown service");

        Ok(())
    }
}
