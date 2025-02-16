use crate::health::health_handler::health_handler;
use common_config::ServiceID;
use common_exchange::ExchangeID;
use common_service::{print_utils, shutdown_utils};
use ims_iggy_config;
use tokio::time::Instant;
use trait_data_integration::ImsDataIntegration;
use warp::Filter;

use crate::handle::utils;
use crate::service::Service;
use crate::stop;
use config_manager::{ConfigManager, ConfigManagerTrait};

pub async fn start<Integration>(
    dbg: bool,
    exchange_id: ExchangeID,
    ims_integration: Integration,
) -> Result<(), Box<dyn std::error::Error>>
where
    Integration: ImsDataIntegration + Send + Sync + 'static,
{
    let start = Instant::now();

    let dbg_print = |msg: &str| {
        if dbg {
            println!("[{exchange_id}]: {msg}");
        }
    };

    dbg_print("build config manager");
    let cfg_manager = if dbg {
        ConfigManager::default_with_debug()
    } else {
        ConfigManager::default()
    };

    dbg_print("build config files");
    let integration_config = &ims_iggy_config::ims_data_integration_config(exchange_id);
    let data_integration = integration_config.integration_id();
    let svc_name = &format!("IMS {data_integration} Service");
    let env = cfg_manager.env_type();
    dbg_print(&format!("Detected Environment: {}", &env));

    dbg_print("Configure service ip and port automatically!");
    let service_addr = cfg_manager.data_svc_socket_addr(exchange_id)?;

    dbg_print("Construct iggy client");
    let iggy_config = &ims_iggy_config::ims_control_iggy_config(exchange_id);
    let iggy_client = utils::build_iggy_client(integration_config.iggy_url()).await?;

    dbg_print("Configure health check route");
    // curl http://localhost:PORT/health
    let health_check = warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(health_handler);

    dbg_print("Configure service routes");
    let routes = health_check;

    dbg_print("Configure http service");
    let port_http = cfg_manager
        .data_svc_port(exchange_id)
        .expect("Failed to get port");
    let http_signal = shutdown_utils::signal_handler("http server");
    let (_, http_server) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], port_http), http_signal);

    dbg_print("Construct server");
    let server = Service::build_service(
        dbg,
        &iggy_client,
        ims_integration,
        integration_config,
        iggy_config,
    )
    .await?;

    dbg_print("Starting message service");
    let service_signal = shutdown_utils::signal_handler("messaging server");
    let service_handle = tokio::spawn(server.run(service_signal));
    dbg_print("Starting http server");
    let http_handle = tokio::spawn(http_server);

    print_utils::print_duration("Starting service took:", &start.elapsed());
    print_utils::print_start_header_simple(svc_name, &service_addr);
    match tokio::try_join!(http_handle, service_handle) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("IMS Data Integration Service: Failed to start server: {e:?}");
        }
    }

    stop::shutdown_iggy(
        &dbg_print,
        &iggy_config.stream_id(),
        &iggy_config.topic_id(),
        &iggy_client,
    )
    .await;

    print_utils::print_stop_header(&ServiceID::Default);

    Ok(())
}
