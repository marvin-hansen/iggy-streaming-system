use crate::service::Service;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration> Service<Integration> {
    pub(crate) async fn shutdown(&self) -> Result<(), std::fmt::Error> {
        let client_db = self.client_producers().read().await;
        if !client_db.is_empty() {
            self.dbg_print("Logging out all remaining clients");
            for (client_id, _) in client_db.iter() {
                self.client_logout(*client_id)
                    .await
                    .unwrap_or_else(|_| panic!("Failed to log out client {client_id}"));
            }
        }

        self.dbg_print("Shutdown integration service");
        self.ims_integration()
            .read()
            .await
            .shutdown()
            .await
            .expect("Failed to shutdown integration service");

        Ok(())
    }
}
