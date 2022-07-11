use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addresses: Vec<TcpListener> = ["127.0.0.1:8000", "::1:8000"]
        .map(|address| TcpListener::bind(address).unwrap())
        .into();
    run(addresses)?.await
}
