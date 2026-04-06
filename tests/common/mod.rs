pub mod mock_sglang;
pub mod seed;

use sqlx::PgPool;

pub struct TestContext {
    pub client: reqwest::Client,
    pub app_url: String,
    pub pool: PgPool,
}

async fn connect_pool(max_connections: u32) -> PgPool {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url)
        .await
        .expect("Failed to connect to DB")
}

/// Starts the mock LLM server and the real app on random ports.
/// Truncates the candidates table so each test starts clean.
/// Must be called with --test-threads=1 since it sets LLM_URL env var.
/// Tests must use #[tokio::test(flavor = "current_thread")].
pub async fn setup() -> TestContext {
    // Test pool — used for truncation and seeding
    let pool = connect_pool(2).await;

    sqlx::query("TRUNCATE TABLE candidates RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await
        .expect("Failed to truncate candidates");

    // Start mock LLM server on a random port
    let mock_listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let mock_addr = mock_listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::Server::from_tcp(mock_listener)
            .unwrap()
            .serve(mock_sglang::router().into_make_service())
            .await
            .unwrap();
    });

    // Point LLM client at the mock.
    // Safety: tests run with --test-threads=1 and current_thread tokio flavour,
    // so no concurrent env var mutation occurs.
    unsafe {
        std::env::set_var("LLM_URL", format!("http://{}", mock_addr));
    }

    // App pool — separate from test pool so app requests don't exhaust test connections
    let app_pool = connect_pool(2).await;

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

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    TestContext {
        client: reqwest::Client::new(),
        app_url: format!("http://{}", app_addr),
        pool,
    }
}
