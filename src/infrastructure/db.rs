use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn get_pool() -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    //use env variable in real production project
    let database_url = format!("postgres://root:root@localhost:5432/learning");
    match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            Ok(pool)
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            Err(err.into())
        }
    }
}

// Alias pour la fonction attendue dans main.rs
pub async fn create_database() -> Pool<Postgres> {
    get_pool().await.expect("Failed to create database connection")
}