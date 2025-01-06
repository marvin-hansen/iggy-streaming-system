use std::future::Future;
use std::net::SocketAddr;
use warp::Filter;

pub(crate) async fn get_http_health_server(
    server_address: String,
) -> impl Future<Output = ()> + Sized {
    let health_uri = "health";
    let routes = warp::get()
        .and(warp::path(health_uri))
        .and(warp::path::end())
        .and_then(health_handler);

    let http_addr: SocketAddr = server_address
        .parse()
        .expect("[DBGW/main]: Failed to parse address");

    warp::serve(routes).bind(http_addr)
}

async fn health_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let result = { String::from("Online") };
    Ok(warp::reply::json(&result))
}
