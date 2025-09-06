use axum::{routing::get, Router};
use tokio::net::TcpListener;
use std::net::SocketAddr;

mod domain;
mod infrastructure;
mod interfaces;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::serve(TcpListener::bind(addr).await.unwrap(), app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, world!"
}
