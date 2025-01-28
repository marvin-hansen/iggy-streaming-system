use crate::MessageConsumer;
use std::fmt::Error;

impl MessageConsumer {
    pub(crate) async fn shutdown(&mut self) -> Result<(), Error> {
        self.dbg_print("Shutdown message consumer");
        // self.consumer.shutdown().await;

        Ok(())
    }
}
