pub(crate) async fn health_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let result = { String::from("online") };
    Ok(warp::reply::json(&result))
}
