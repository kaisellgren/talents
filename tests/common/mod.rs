pub mod mock_llm;
pub mod seed;

use sqlx::PgPool;
use std::sync::OnceLock;

pub struct TestServer {
    pub app_url: String,
    pub pool: PgPool,
}

static SERVER: OnceLock<TestServer> = OnceLock::new();

/// Returns the shared test server, starting mock LLM + app on first call.
/// Servers run on a dedicated background thread with their own Tokio runtime,
/// so they survive across test runtimes.
pub async fn shared_server() -> &'static TestServer {
    if let Some(s) = SERVER.get() {
        return s;
    }

    let (tx, rx) = std::sync::mpsc::channel::<TestServer>();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            dotenvy::dotenv().ok();
            let database_url =
                std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

            // Start mock LLM server
            let mock_listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            let mock_addr = mock_listener.local_addr().unwrap();
            tokio::spawn(async move {
                axum::Server::from_tcp(mock_listener)
                    .unwrap()
                    .serve(mock_llm::router().into_make_service())
                    .await
                    .unwrap();
            });

            unsafe {
                std::env::set_var("LLM_URL", format!("http://{}", mock_addr));
            }

            // Test pool for seeding
            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
                .expect("Failed to connect test pool");

            // App pool
            let app_pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
                .expect("Failed to connect app pool");

            // Start real app
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

            tx.send(TestServer {
                app_url: format!("http://{}", app_addr),
                pool,
            })
            .unwrap();

            // Keep this runtime alive forever so servers keep running
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
            }
        });
    });

    let server = rx.recv().expect("Failed to start test server");
    SERVER.get_or_init(|| server)
}
