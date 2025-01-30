use iggy::clients::client::IggyClient;
/// Shuts down iggy clients and user.
///
/// # Parameters
///
/// * `dbg_print` - A function to print debug messages.
/// * `producer_client` - The iggy producer client.
/// * `consumer_client` - The iggy consumer client.
pub(crate) async fn shutdown_iggy(
    dbg_print: &dyn Fn(&str),
    producer_client: &IggyClient,
    consumer_client: &IggyClient,
) {
    // dbg_print("Shutdown iggy producer");
    // dbg_print("Deleting producer streams and topics");
    // message_shared::cleanup(&producer_client, iggy_config)
    //     .await
    //     .expect("Failed to clean up iggy");

    dbg_print("Logging out iggy user");
    message_shared::logout_user(producer_client)
        .await
        .expect("Failed to logout user");

    dbg_print("Shutting down iggy producer client");
    message_shared::shutdown(producer_client)
        .await
        .expect("Failed to shutdown iggy producer client");

    dbg_print("Shutdown iggy consumer client");
    message_shared::shutdown(consumer_client)
        .await
        .expect("Failed to shutdown iggy consumer client");
}
