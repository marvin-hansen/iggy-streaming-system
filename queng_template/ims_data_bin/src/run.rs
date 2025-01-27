use crate::service::ImsDataService;
use common_errors::MessageProcessingError;
use trait_data_integration::ImsDataIntegration;

impl<Integration> ImsDataService<Integration>
where
    Integration: ImsDataIntegration,
{
    pub async fn run(self) -> Result<(), MessageProcessingError> {
        tokio::spawn(async move {
            println!("Running data integration service");
            //
            // Implement your run logic here i.e. listen for incoming messages
            //
        });

        Ok(())
    }
}
