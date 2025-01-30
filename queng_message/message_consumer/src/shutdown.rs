use crate::MessageConsumer;
use std::fmt::Error;

impl MessageConsumer {
    pub(crate) async fn shutdown(&self) -> Result<(), Error> {
        self.dbg_print("Shutdown message consumer");
        // let handler = &self.consumer_handle;
        // match handler {
        //     None => {}
        //     Some(consumer_handle) => {
        //         consumer_handle.abort();
        //     }
        // }

        Ok(())
    }
}
