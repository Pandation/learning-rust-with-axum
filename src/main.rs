use tokio::net::TcpListener;
use std::net::SocketAddr;

use crate::interfaces::http::routes::create_router;
use crate::infrastructure::db::create_database;
use crate::infrastructure::app_state::AppState;

mod domain;
mod infrastructure;
mod interfaces;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = create_database().await;
    let app = create_router()
    .with_state(AppState::new(database));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::serve(TcpListener::bind(addr).await.unwrap(), app).await?;

    Ok(())
}
