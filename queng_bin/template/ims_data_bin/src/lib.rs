use tokio::signal::unix::{signal, SignalKind};
use trait_data_integration::ImsDataIntegration;
mod health_check;
mod run;
mod service;
mod symbols;

pub async fn start<Integration>(
    dbg: bool,
    svc_name: &str,
    ims_integration: Integration,
) -> Result<(), Box<dyn std::error::Error>>
where
    Integration: ImsDataIntegration,
{
    let dbg_print = |msg: &str| {
        if dbg {
            println!("[{svc_name}]: {msg}");
        }
    };

    dbg_print("Construct health endpoint server");
    let health_addr = String::from("127.0.0.1:8080");
    let http_server = health_check::health_handler::get_http_health_server(health_addr).await;

    dbg_print("Construct Integration server");
    let server = service::ImsDataService::new(ims_integration);

    // The fetch and validate steps below only exists to demonstrate access to the
    // the generic integration. In practice, you may remove these steps or mute the output
    // unless an error occurs i.e. exchange offline.
    dbg_print("Fetching exchange symbols");
    let res = server.get_exchange_symbols().await;
    if res.is_ok() {
        let symbols = res.unwrap();
        println!("Fetched {} symbols", symbols.len());

        let v: Vec<_> = symbols.into_iter().collect();
        let res = server.validate_symbols(&v).await;
        if res.is_ok() {
            let valid = res.unwrap();
            println!("All symbols valid. {}", valid);
        } else {
            println!("Failed to validate symbols");
            println!("{}", res.unwrap_err());
        }
    }

    server.run().await.expect("Failed to run service");
    http_server.await;

    #[cfg(unix)]
    let (mut ctrl_c, mut sigterm, mut sighang, mut sigquit) = {
        (
            signal(SignalKind::interrupt())?,
            signal(SignalKind::terminate())?,
            signal(SignalKind::hangup())?,
            signal(SignalKind::quit())?,
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
        _ = sighang.recv() => {
            dbg_print("Received HANGUP. Shutting down {NAME} {VERSION}...");
        }
        _ = sigquit.recv() => {
            dbg_print("Received QUIT. Shutting down {NAME} {VERSION}...");
        }
    }

    Ok(())
}
