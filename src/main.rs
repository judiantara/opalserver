use opalserver::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    println!("Starting server at 0.0.0.0:8080");
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind 127.0.0.1:8080");
    run(listener)?.await
}
