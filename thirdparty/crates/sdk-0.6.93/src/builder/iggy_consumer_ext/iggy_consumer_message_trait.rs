use async_trait::async_trait;

use crate::builder::EventConsumer;
use iggy::error::IggyError;
use tokio::sync::oneshot;

#[async_trait]
pub trait IggyConsumerMessageExt {
    async fn consume_messages(
        mut self,
        event_processor: &'static (impl EventConsumer + Sync),
        shutdown_rx: oneshot::Receiver<()>, // or any `Future<Output=()>`
    ) -> Result<(), IggyError>;
}
