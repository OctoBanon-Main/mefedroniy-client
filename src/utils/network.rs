use tokio::net::lookup_host;

pub async fn resolve_address(address: &str, port: &str) -> Option<String> {
    let addr = format!("{}:{}", address, port);
    lookup_host(addr).await.ok()
        .and_then(|mut addrs| addrs.next())
        .map(|a| a.to_string())
}