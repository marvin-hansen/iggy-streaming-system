use iggy::clients::client::IggyClient;
use iggy::identifier::Identifier;
use iggy::locking::IggySharedMutFn;

/// Shuts down iggy clients and user.
///
/// # Parameters
///
/// * `dbg_print` - A function to print debug messages.
/// * `producer_client` - The iggy producer client.
/// * `consumer_client` - The iggy consumer client.
pub(crate) async fn shutdown_iggy(
    dbg_print: &dyn Fn(&str),
    control_stream_id: &Identifier,
    control_topic_id: &Identifier,
    producer_client: &IggyClient,
    consumer_client: &IggyClient,
) {
    dbg_print("Delete control topic");
    consumer_client
        .client()
        .read()
        .await
        .delete_topic(control_stream_id, control_topic_id)
        .await
        .expect("Failed to delete control topic");

    dbg_print("Delete control stream");
    consumer_client
        .client()
        .read()
        .await
        .delete_stream(control_stream_id)
        .await
        .expect("Failed to control data topic");

    dbg_print("Shutting down iggy producer client");
    message_shared::shutdown(producer_client)
        .await
        .expect("Failed to shutdown iggy producer client");

    dbg_print("Shutdown iggy consumer client");
    message_shared::shutdown(consumer_client)
        .await
        .expect("Failed to shutdown iggy consumer client");
}
