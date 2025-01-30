use crate::MessageConsumer;
use futures_util::stream::StreamExt;
use tokio::select;
use tokio::sync::oneshot;
use trait_event_consumer::EventConsumer;

//
// https://discord.com/channels/1144142576266530928/1144142577369628684/1333360421842980866
impl MessageConsumer {
    pub async fn consume_messages(
        self,
        data_event_processor: &'static (impl EventConsumer + Sync),
        shutdown_rx: oneshot::Receiver<()>, // or any `Future<Output=()>`
    ) {
        tokio::spawn(async move {
            let consumer = &mut self.consumer.write().await;

            select! {
                _ = shutdown_rx => {
                    self.dbg_print("Received shutdown signal");
                    self.shutdown()
                        .await
                    .expect("[MessageConsumer]: Failed to shutdown");
                }

                received_message = consumer.next() => {
                    match received_message {
                        Some(Ok(message)) => data_event_processor
                            .consume(message.message.payload.to_vec())
                            .await
                            .expect("[MessageConsumer]: Failed to process message"),

                        Some(Err(err)) =>   {
                            self.handle_iggy_error(err)
                            .await
                            .expect("[MessageConsumer]: Failed to handle error");
                        },

                        None => {},
                    }
                }

            }
        });
    }
}
