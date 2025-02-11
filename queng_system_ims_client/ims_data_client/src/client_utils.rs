use crate::ImsClientError;
use iggy::client::Client;
use iggy::clients::client::IggyClient;

pub(crate) async fn build_iggy_client(
    connection_string: &str,
) -> Result<IggyClient, ImsClientError> {
    let iggy_client = match IggyClient::from_connection_string(connection_string) {
        Ok(client) => client,
        Err(err) => {
            return Err(ImsClientError::FailedToCreateIggyClient(err.to_string()));
        }
    };
    match iggy_client.connect().await {
        Ok(_) => (),
        Err(err) => {
            return Err(ImsClientError::FailedToConnectToIggyServer(err.to_string()));
        }
    };

    Ok(iggy_client)
}
