use trait_data_integration::ImsDataIntegration;

mod health_check;

pub async fn start(
    dbg: bool,
    svc_name: &str,
    ims_integration: impl ImsDataIntegration,
) -> Result<(), Box<dyn std::error::Error>> {
    let dbg_print = |msg: &str| {
        if dbg {
            println!("[{svc_name}]: {msg}");
        }
    };

    dbg_print("Construct health endpoint server");
    let health_addr = String::from("127.0.0.1:8080");
    let http_server = health_check::health_handler::get_http_health_server(health_addr).await;

    dbg_print("Fetching exchange symbols");
    let res = ims_integration.get_exchange_symbols().await;
    if res.is_ok() {
        let symbols = res.unwrap();
        println!("Serving {} symbols", symbols.len());
    }

    http_server.await;

    #[cfg(unix)]
    let (mut ctrl_c, mut sigterm) = {
        use tokio::signal::unix::{signal, SignalKind};
        (
            signal(SignalKind::interrupt())?,
            signal(SignalKind::terminate())?,
        )
    };

    #[cfg(unix)]
    tokio::select! {
        _ = ctrl_c.recv() => {
            dbg_print("Received SIGINT. Shutting down {NAME} {VERSION}...");
        },
        _ = sigterm.recv() => {
            dbg_print("Received SIGTERM. Shutting down {NAME} {VERSION}...");
        }
    }

    Ok(())
}
