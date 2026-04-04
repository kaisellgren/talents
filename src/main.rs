use axum::Router;
use axum::Extension;
use sqlx::PgPool;
use std::net::SocketAddr;

mod agents;
mod db;
mod sglang;
mod routes {
    pub mod candidate;
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    let app = Router::new()
        .nest("/candidates", routes::candidate::router())
        .route("/agents/run", axum::routing::post(routes::candidate::run_agent))
        .layer(Extension(pool));

    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
