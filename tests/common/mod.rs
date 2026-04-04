pub mod mock_sglang;
pub mod seed;

use sqlx::PgPool;

pub struct TestContext {
    pub client: reqwest::Client,
    pub app_url: String,
    pub pool: PgPool,
}

/// Creates a fresh DB pool for use within the current tokio runtime.
/// A new pool is created per test because sqlx pools bind their background tasks
/// to the tokio runtime that created them; reusing a pool across runtimes breaks it.
async fn get_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to DB")
}

/// Starts the mock SGLang server and the real app on random ports.
/// Truncates the candidates table so each test starts clean.
/// Must be called with --test-threads=1 since it sets SGLANG_URL env var.
/// Tests should use #[tokio::test(flavor = "current_thread")] to avoid multi-threaded env mutation.
pub async fn setup() -> TestContext {
    let pool = get_pool().await;

    // Clean state before each test
    sqlx::query("TRUNCATE TABLE candidates RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await
        .expect("Failed to truncate candidates");

    // Start mock SGLang on a random port
    let mock_listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let mock_addr = mock_listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::Server::from_tcp(mock_listener)
            .unwrap()
            .serve(mock_sglang::router().into_make_service())
            .await
            .unwrap();
    });

    // Point sglang client at the mock.
    // Safety: tests run with --test-threads=1 and current_thread tokio flavour,
    // so no concurrent env var mutation occurs.
    unsafe {
        std::env::set_var("SGLANG_URL", format!("http://{}", mock_addr));
    }

    // Create a dedicated pool for the app so it doesn't exhaust the shared test pool.
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let app_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create app pool");

    // Start real app on a random port using its own pool
    let app_listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let app_addr = app_listener.local_addr().unwrap();
    let app = talents::create_app(app_pool);
    tokio::spawn(async move {
        axum::Server::from_tcp(app_listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    // Give servers a moment to be ready
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    TestContext {
        client: reqwest::Client::new(),
        app_url: format!("http://{}", app_addr),
        pool: pool.clone(),
    }
}
