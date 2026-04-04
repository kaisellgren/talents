use std::net::SocketAddr;
use sqlx::PgPool;

mod agents;
mod db;
mod routes;
mod sglang;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    let app = talents::create_app(pool);

    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
