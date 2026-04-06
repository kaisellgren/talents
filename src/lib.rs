pub mod agents;
pub mod db;
pub mod routes;
pub mod sglang;

use axum::{routing::{get, post}, Extension, Router};
use sqlx::PgPool;

/// Builds the Axum router with DB pool injected. Used by both main() and integration tests.
pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(routes::health::handler))
        .nest("/talents", routes::talent::router())
        .route("/agents/run", post(routes::talent::run_agent))
        .layer(Extension(pool))
}
