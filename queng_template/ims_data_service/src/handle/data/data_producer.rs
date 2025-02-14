use crate::service::Service;
use common_errors::MessageProcessingError;
use iggy::clients::producer::IggyProducer;
use std::sync::Arc;
use trait_data_integration::ImsDataIntegration;

impl<Integration: ImsDataIntegration> Service<Integration> {
    pub(crate) async fn get_data_producer(
        &self,
        client_id: u16,
    ) -> Result<Arc<IggyProducer>, MessageProcessingError> {
        let client_data_producers = self.client_producers().write().await;

        let exists = match self.check_client_login(client_id).await {
            Ok(exists) => exists,
            Err(err) => {
                return Err(MessageProcessingError(format!(
                    "Failed to check if client with id {} is logged in due to error: {}",
                    client_id, err
                )));
            }
        };

        if !exists {
            return Err(MessageProcessingError(format!(
                "Client with id {} is not logged in",
                client_id
            )));
        }

        match client_data_producers.get(&client_id) {
            Some(data_producer) => Ok(data_producer.clone()),
            None => Err(MessageProcessingError(format!(
                "Client with id {} does not have a data producer",
                client_id
            ))),
        }
    }
}
