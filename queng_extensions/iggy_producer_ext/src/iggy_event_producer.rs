use crate::EventProducer;
use bytes::Bytes;
use iggy::clients::producer::IggyProducer;
use iggy::error::IggyError;
use iggy::messages::send_messages::Message;

impl EventProducer for IggyProducer {
    async fn send_one_event(&self, message: Vec<u8>) -> Result<(), IggyError> {
        let payload = Bytes::from(message);
        let message = Message::new(None, payload, None);

        self.send_one(message).await
    }

    async fn send_event_batch(&self, messages: Vec<Vec<u8>>) -> Result<(), IggyError> {
        // Convert Vec<Vec<u8>> to Vec<Message>
        let messages = messages
            .iter()
            .map(|message| {
                let payload = Bytes::from(message.clone());
                Message::new(None, payload, None)
            })
            .collect();

        self.send(messages).await
    }
}
