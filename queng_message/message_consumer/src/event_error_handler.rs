use crate::MessageConsumer;
use iggy::error::IggyError;
use std::fmt::Error;

impl MessageConsumer {
    pub(crate) async fn handle_iggy_error(&mut self, err: IggyError) -> Result<(), Error> {
        match err {
            IggyError::Disconnected => {
                self.dbg_print("Disconnected:  shutdown client");
                self.shutdown().await
            }
            IggyError::CannotEstablishConnection => {
                self.dbg_print("CannotEstablishConnection:  shutdown client");
                self.shutdown().await
            }
            IggyError::StaleClient => {
                self.dbg_print("StaleClient:  shutdown client");
                self.shutdown().await
            }
            IggyError::InvalidServerAddress => {
                self.dbg_print("InvalidServerAddress:  shutdown client");
                self.shutdown().await
            }
            IggyError::InvalidClientAddress => {
                self.dbg_print("InvalidClientAddress:  shutdown client");
                self.shutdown().await
            }
            IggyError::NotConnected => {
                self.dbg_print("NotConnected:  shutdown client");
                self.shutdown().await
            }
            IggyError::ClientShutdown => {
                self.dbg_print("ClientShutdown:  shutdown client");
                self.shutdown().await
            }
            _ => {
                eprintln!(
                    "[MessageConsumer]: Error polling messages from iggy message bus: {}",
                    err
                );

                Ok(())
            }
        }
    }
}
