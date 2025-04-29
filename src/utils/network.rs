use std::net::ToSocketAddrs;

pub async fn resolve_address(address: &str, port: &str) -> Option<String> {
    let addr = format!("{}:{}", address, port);
    addr.to_socket_addrs()
        .ok()
        .and_then(|mut iter| iter.next())
        .map(|a| a.to_string())
}
