use iggy::client::{Client, StreamClient, TopicClient, UserClient};
use iggy::clients::client::IggyClient;
use iggy::identifier::Identifier;
use std::error::Error;

/// Cleans up an Iggy client, deleting the topic and stream.
///
/// # Arguments
///
/// * `client` - The Iggy client to clean up.
/// * `iggy_config` - The configuration to use to clean up the client.
///
/// # Returns
///
/// A `Result` containing a `()` on success or an error on failure.
///
pub async fn cleanup(
    client: &IggyClient,
    stream_id: &Identifier,
    topic_id: &Identifier,
) -> Result<(), Box<dyn Error>> {
    match client.delete_topic(stream_id, topic_id).await {
        Ok(_) => (),
        Err(err) => return Err(Box::from(err)),
    }

    match client.delete_stream(stream_id).await {
        Ok(_) => (),
        Err(err) => return Err(Box::from(err)),
    }

    Ok(())
}

/// Logs out an Iggy user client.
///
/// # Arguments
///
/// * `client` - The Iggy user client to log out.
///
/// # Returns
///
/// A `Result` containing a `()` on success or an error on failure.
///
pub async fn logout_user(client: &IggyClient) -> Result<(), Box<dyn Error>> {
    match client.logout_user().await {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::from(err)),
    }
}
/// Shuts down an Iggy client, disconnecting it from the server.
///
/// # Arguments
///
/// * `client` - The Iggy client to shut down.
///
/// # Returns
///
/// A `Result` containing a `()` on success or an error on failure.
///
pub async fn shutdown(client: &IggyClient) -> Result<(), Box<dyn Error>> {
    client
        .shutdown()
        .await
        .expect("Failed to shutdown iggy server");

    client
        .disconnect()
        .await
        .expect("Failed to connect to iggy server");

    Ok(())
}
