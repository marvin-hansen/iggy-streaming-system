use tokio_tungstenite::{connect_async, WebSocketStream};
use trait_data_integration::ImsDataIntegrationError;

/// Connects to the Binance WebSocket server with the specified stream name.
///
/// Returns a `WebSocketStream` that can be used to send and receive messages.
///
/// # Arguments
/// * `stream_name` - The name of the stream to connect to (e.g., "!ticker@arr")
/// * `url` - The URL of the WebSocket server
///
/// # Errors
/// * `MessageProcessingError`: If the connection fails or the stream is not found
///
pub(crate) async fn connect_websocket_static(
    stream_name: &str,
    url: String,
) -> Result<
    WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    ImsDataIntegrationError,
> {
    let url = format!("{}/{}", url, stream_name);
    let (ws_stream, _) = connect_async(&url).await.map_err(|e| {
        ImsDataIntegrationError::FailedToConnectToWebSocket(format!(
            "WebSocket connection error: {}",
            e
        ))
    })?;
    Ok(ws_stream)
}
