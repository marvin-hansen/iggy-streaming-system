use crate::MessageConsumer;
use futures_util::stream::StreamExt;
use trait_event_consumer::EventConsumer;
//
// https://discord.com/channels/1144142576266530928/1144142577369628684/1333360421842980866
impl MessageConsumer {
    pub fn consume_messages(
        mut self,
        data_event_processor: &'static (impl EventConsumer + Send + Sync),
    ) {
        self.dbg_print("consume_messages");

        tokio::spawn(async move {
            let consumer = &mut self.consumer;

            while let Some(received_message) = consumer.next().await {
                match received_message {
                    Ok(message) => data_event_processor
                        .consume(message.message.payload.to_vec())
                        .await
                        .expect("[MessageConsumer]: Failed to process message"),
                    Err(e) => {
                        eprintln!(
                            "[MessageConsumer]: Error polling messages from iggy message bus: {}",
                            e
                        );
                    }
                }
            }
        });
    }
}
