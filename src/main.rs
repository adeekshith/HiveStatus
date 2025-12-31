use hive_status::configuration::AppConfig;
use hive_status::startup;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use tokio::net::TcpListener; // Use tokio's TcpListener

#[tokio::main]
async fn main() {
    let config = AppConfig::new();

    let addr = SocketAddr::new(
        IpAddr::from_str(&config.host).unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
        config.port,
    );
    let listener = TcpListener::bind(addr).await.expect("Failed to bind to specified address");

    startup::run_with_listener(config, listener).await;
}
