use common_config::ServiceID;
use std::time::Duration;

pub fn print_start_header_simple(service_name: &str, service_addr: &str) {
    println!();
    println!("||  {service_name}  ||");
    println!("==========================================");
    println!("Service on endpoint: {service_addr}");
    println!("==========================================");
    println!();
}

pub fn print_start_header(
    service_id: &ServiceID,
    service_addr: &str,
    metrics_addr: &str,
    metrics_uri: &str,
    health_uri: &str,
) {
    println!("||  {service_id}  ||");
    println!("==========================================");
    println!("Service on endpoint: {service_addr}");
    println!("Metrics on endpoint: {metrics_addr}/{metrics_uri}");
    println!("Health  on endpoint: {metrics_addr}/{health_uri}");
    println!("==========================================");
    println!();
}

pub fn print_start_header_message_service(service_name: &str, service_topic: &str) {
    println!("||  {service_name}  ||");
    println!("==========================================");
    println!("Listening on topic: {service_topic}");
    println!("==========================================");
    println!();
}

pub fn print_stop_header(service_id: &ServiceID) {
    println!();
    println!("==========================================");
    println!("{service_id} service shutdown complete");
    println!("==========================================");
}

pub fn print_duration(msg: &str, elapsed: &Duration) {
    if elapsed.as_millis() > 1000 {
        println!("{} {} sec.", msg, elapsed.as_secs());
    } else {
        println!("{} {} ms.", msg, elapsed.as_millis());
    }
}
