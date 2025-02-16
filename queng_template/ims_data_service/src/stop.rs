use iggy::client::Client;
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
    iggy_client: &IggyClient,
) {
    dbg_print("Shutting down messaging service");

    dbg_print("Delete control topic");
    iggy_client
        .client()
        .read()
        .await
        .delete_topic(control_stream_id, control_topic_id)
        .await
        .expect("Failed to delete control topic");

    dbg_print("Delete control stream");
    iggy_client
        .client()
        .read()
        .await
        .delete_stream(control_stream_id)
        .await
        .expect("Failed to control data topic");

    dbg_print("Shutting down iggy consumer client");
    iggy_client
        .shutdown()
        .await
        .expect("Failed to shutdown iggy consumer client");
}
