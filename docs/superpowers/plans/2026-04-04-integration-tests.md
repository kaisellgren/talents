# Integration Tests Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Write integration tests for all HTTP endpoints using a real Postgres database and a lightweight mock SGLang server that returns canned responses.

**Architecture:** A single test binary (`tests/api.rs`) backed by shared helpers in `tests/common/`. A `create_app()` function is extracted to `src/lib.rs` so tests can spin up the real Axum app on a random port. The mock SGLang server inspects the system prompt keyword to return the appropriate canned JSON. Each test truncates the DB before use; tests run with `--test-threads=1`.

**Tech Stack:** Rust, axum 0.6, sqlx, reqwest, tokio, serde_json

---

## File Map

| File | Action | Responsibility |
|------|--------|----------------|
| `src/lib.rs` | Modify | Add `pub mod routes`, `pub fn create_app(pool)` |
| `src/routes/mod.rs` | Create | Declare `pub mod candidate;` (moves module out of main.rs inline block) |
| `src/main.rs` | Modify | Replace inline `mod routes { pub mod candidate; }` with `mod routes;` |
| `tests/common/mod.rs` | Create | `TestContext`, `setup()`, DB cleanup |
| `tests/common/mock_sglang.rs` | Create | Mock SGLang Axum server |
| `tests/common/seed.rs` | Create | `seed_candidate()` helper |
| `tests/api.rs` | Create | All 6 integration tests |

---

## Task 1: Expose `routes` module from library crate and add `create_app`

Tests need to build the real Axum app. To do that without duplicating router setup, `create_app(pool)` lives in `src/lib.rs`. This requires moving the `routes` inline module declaration out of `main.rs`.

**Files:**
- Create: `src/routes/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Create `src/routes/mod.rs`**

```rust
pub mod candidate;
```

- [ ] **Step 2: Update `src/lib.rs`**

```rust
pub mod agents;
pub mod db;
pub mod routes;
pub mod sglang;

use axum::{routing::post, Extension, Router};
use sqlx::PgPool;

/// Builds the Axum router with DB pool injected. Used by both main() and integration tests.
pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .nest("/candidates", routes::candidate::router())
        .route("/agents/run", post(routes::candidate::run_agent))
        .layer(Extension(pool))
}
```

- [ ] **Step 3: Update `src/main.rs`**

Replace the inline `mod routes { pub mod candidate; }` block and the manual router construction:

```rust
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
```

- [ ] **Step 4: Verify compilation**

```bash
cargo check
```

Expected: no errors.

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs src/routes/mod.rs src/main.rs
git commit -m "refactor: expose create_app from lib crate for integration tests"
```

---

## Task 2: Create mock SGLang server (`tests/common/mock_sglang.rs`)

**Files:**
- Create: `tests/common/mock_sglang.rs`

- [ ] **Step 1: Create `tests/common/mock_sglang.rs`**

```rust
use axum::{routing::post, Json, Router};
use axum::http::StatusCode;
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new().route("/v1/chat/completions", post(handle_completion))
}

async fn handle_completion(
    Json(body): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let system_content = body["messages"]
        .as_array()
        .and_then(|msgs| msgs.iter().find(|m| m["role"] == "system"))
        .and_then(|m| m["content"].as_str())
        .unwrap_or("");

    let user_content = body["messages"]
        .as_array()
        .and_then(|msgs| msgs.iter().find(|m| m["role"] == "user"))
        .and_then(|m| m["content"].as_str())
        .unwrap_or("");

    let response_content = if system_content.contains("triage") {
        r#"{"required_skills":["rust"],"preferred_skills":[],"location_city":null,"location_country":null,"max_hourly_rate":null}"#
            .to_string()
    } else if system_content.contains("ranking") {
        let candidates = parse_candidates_from_user_content(user_content);
        let rankings: Vec<Value> = candidates
            .iter()
            .map(|id| json!({"candidate_id": id, "score": 0.9, "reasoning": "Strong match"}))
            .collect();
        serde_json::to_string(&json!({"rankings": rankings})).unwrap()
    } else if system_content.contains("summarizer") {
        let candidates = parse_candidates_from_user_content(user_content);
        let summaries: Vec<Value> = candidates
            .iter()
            .map(|id| json!({"candidate_id": id, "summary": "Great candidate."}))
            .collect();
        serde_json::to_string(&json!({"summaries": summaries})).unwrap()
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };

    Ok(Json(json!({
        "choices": [{"message": {"content": response_content}}]
    })))
}

/// Parses candidate UUIDs from the user message content.
/// The format is: "Prompt: ...\n\nCandidates: [{"id": "uuid", ...}]"
fn parse_candidates_from_user_content(user_content: &str) -> Vec<String> {
    let candidates_part = user_content
        .split("Candidates: ")
        .nth(1)
        .unwrap_or("[]");
    let candidates: Vec<Value> = serde_json::from_str(candidates_part).unwrap_or_default();
    candidates
        .iter()
        .filter_map(|c| c["id"].as_str().map(String::from))
        .collect()
}
```

---

## Task 3: Create seed helper (`tests/common/seed.rs`)

**Files:**
- Create: `tests/common/seed.rs`

- [ ] **Step 1: Create `tests/common/seed.rs`**

```rust
use sqlx::PgPool;
use talents::db::candidate::Candidate;
use uuid::Uuid;
use chrono::Utc;

pub struct CandidateOverrides {
    pub name: Option<String>,
    pub skills: Option<Vec<String>>,
    pub available: Option<bool>,
    pub hourly_rate_max: Option<i32>,
}

impl Default for CandidateOverrides {
    fn default() -> Self {
        Self {
            name: None,
            skills: None,
            available: None,
            hourly_rate_max: None,
        }
    }
}

/// Inserts a candidate into the DB with sensible defaults, accepting field overrides.
/// Returns the inserted Candidate with its DB-assigned ID and created_at.
pub async fn seed_candidate(pool: &PgPool, overrides: CandidateOverrides) -> Candidate {
    let skills = overrides.skills.unwrap_or_else(|| vec!["rust".into(), "postgresql".into()]);
    let skills_json = serde_json::to_value(&skills).unwrap();

    sqlx::query_as::<_, Candidate>(
        r#"
        INSERT INTO candidates (name, skills, location_city, location_country, role, available, hourly_rate_min, hourly_rate_max, biography)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(overrides.name.unwrap_or_else(|| "Test Candidate".into()))
    .bind(skills_json)
    .bind("Helsinki")
    .bind("Finland")
    .bind(Option::<String>::None)
    .bind(overrides.available.unwrap_or(true))
    .bind(50_i32)
    .bind(overrides.hourly_rate_max.unwrap_or(100))
    .bind("Experienced developer.")
    .fetch_one(pool)
    .await
    .expect("Failed to seed candidate")
}
```

---

## Task 4: Create `TestContext` and `setup()` (`tests/common/mod.rs`)

**Files:**
- Create: `tests/common/mod.rs`

- [ ] **Step 1: Create `tests/common/mod.rs`**

```rust
pub mod mock_sglang;
pub mod seed;

use sqlx::PgPool;

pub struct TestContext {
    pub client: reqwest::Client,
    pub app_url: String,
    pub pool: PgPool,
}

/// Starts the mock SGLang server and the real app on random ports.
/// Truncates the candidates table so each test starts clean.
/// Must be called with --test-threads=1 since it sets SGLANG_URL env var.
pub async fn setup() -> TestContext {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to DB");

    // Clean state before each test
    sqlx::query("TRUNCATE TABLE candidates")
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

    // Point sglang client at the mock
    // Safety: tests run with --test-threads=1, no concurrent env var mutation
    unsafe {
        std::env::set_var("SGLANG_URL", format!("http://{}", mock_addr));
    }

    // Start real app on a random port
    let app_listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let app_addr = app_listener.local_addr().unwrap();
    let app = talents::create_app(pool.clone());
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
        pool,
    }
}
```

- [ ] **Step 2: Verify the common module compiles (no tests yet)**

```bash
cargo check --tests
```

Expected: no errors.

- [ ] **Step 3: Commit**

```bash
git add tests/common/mod.rs tests/common/mock_sglang.rs tests/common/seed.rs
git commit -m "feat: add test helpers — TestContext, mock SGLang server, seed helper"
```

---

## Task 5: Write candidate API integration tests (`tests/api.rs`)

**Files:**
- Create: `tests/api.rs`

- [ ] **Step 1: Create `tests/api.rs` with the first three tests**

```rust
mod common;

use common::{seed::{seed_candidate, CandidateOverrides}, setup};
use serde_json::{json, Value};

// ─── POST /candidates ─────────────────────────────────────────────────────────

#[tokio::test]
async fn create_candidate_returns_201_with_id() {
    let ctx = setup().await;

    let res = ctx
        .client
        .post(format!("{}/candidates", ctx.app_url))
        .json(&json!({
            "name": "Kai Sellgren",
            "skills": ["rust", "axum"],
            "location_city": "Helsinki",
            "location_country": "Finland",
            "available": true,
            "hourly_rate_min": 80,
            "hourly_rate_max": 120,
            "biography": "Senior Rust engineer."
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 201);
    let body: Value = res.json().await.unwrap();
    assert!(body["id"].as_str().is_some(), "response should contain an id");
}

// ─── GET /candidates/available ────────────────────────────────────────────────

#[tokio::test]
async fn list_available_returns_only_available_candidates() {
    let ctx = setup().await;

    seed_candidate(&ctx.pool, CandidateOverrides { available: Some(true), ..Default::default() }).await;
    seed_candidate(&ctx.pool, CandidateOverrides { available: Some(false), ..Default::default() }).await;

    let res = ctx
        .client
        .get(format!("{}/candidates/available", ctx.app_url))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: Vec<Value> = res.json().await.unwrap();
    assert_eq!(body.len(), 1);
    assert_eq!(body[0]["available"], true);
}

// ─── GET /candidates/search ───────────────────────────────────────────────────

#[tokio::test]
async fn search_by_skill_returns_matching_candidates() {
    let ctx = setup().await;

    seed_candidate(
        &ctx.pool,
        CandidateOverrides {
            skills: Some(vec!["rust".into(), "postgresql".into()]),
            ..Default::default()
        },
    )
    .await;

    let res = ctx
        .client
        .get(format!("{}/candidates/search?skills=rust", ctx.app_url))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: Vec<Value> = res.json().await.unwrap();
    assert_eq!(body.len(), 1);
}

#[tokio::test]
async fn search_without_skills_param_returns_400() {
    let ctx = setup().await;

    let res = ctx
        .client
        .get(format!("{}/candidates/search", ctx.app_url))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
}
```

- [ ] **Step 2: Run these tests**

```bash
cargo test --test api -- --test-threads=1 2>&1
```

Expected: 4 tests pass.

- [ ] **Step 3: Commit**

```bash
git add tests/api.rs
git commit -m "feat: add candidate API integration tests"
```

---

## Task 6: Write agent loop integration tests (`tests/api.rs`)

**Files:**
- Modify: `tests/api.rs`

- [ ] **Step 1: Add agent loop tests to `tests/api.rs`**

Append to the existing file:

```rust
// ─── POST /agents/run ─────────────────────────────────────────────────────────

#[tokio::test]
async fn agent_run_returns_ranked_candidates() {
    let ctx = setup().await;

    // Seed a candidate that matches what the mock triage returns (skills: ["rust"])
    seed_candidate(
        &ctx.pool,
        CandidateOverrides {
            skills: Some(vec!["rust".into()]),
            available: Some(true),
            hourly_rate_max: Some(100),
            ..Default::default()
        },
    )
    .await;

    let res = ctx
        .client
        .post(format!("{}/agents/run", ctx.app_url))
        .json(&json!({"prompt": "I need a Rust developer in Helsinki"}))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: Value = res.json().await.unwrap();
    assert_eq!(body["iterations"], 1);
    let candidates = body["candidates"].as_array().unwrap();
    assert!(!candidates.is_empty(), "expected at least one candidate");
    // Verify the response shape
    let first = &candidates[0];
    assert!(first["id"].as_str().is_some());
    assert!(first["name"].as_str().is_some());
    assert!(first["score"].as_f64().is_some());
    assert!(first["reasoning"].as_str().is_some());
    assert!(first["summary"].as_str().is_some());
}

#[tokio::test]
async fn agent_run_with_no_matching_candidates_retries_and_returns_empty() {
    let ctx = setup().await;
    // No candidates seeded — constraint step will always return empty,
    // triggering 5 retry iterations.

    let res = ctx
        .client
        .post(format!("{}/agents/run", ctx.app_url))
        .json(&json!({"prompt": "Find me a COBOL developer on the Moon"}))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: Value = res.json().await.unwrap();
    assert_eq!(body["iterations"], 5);
    let candidates = body["candidates"].as_array().unwrap();
    assert!(candidates.is_empty(), "expected no candidates after all retries exhausted");
}
```

- [ ] **Step 2: Run all integration tests**

```bash
cargo test --test api -- --test-threads=1 2>&1
```

Expected output:
```
test create_candidate_returns_201_with_id ... ok
test list_available_returns_only_available_candidates ... ok
test search_by_skill_returns_matching_candidates ... ok
test search_without_skills_param_returns_400 ... ok
test agent_run_returns_ranked_candidates ... ok
test agent_run_with_no_matching_candidates_retries_and_returns_empty ... ok

test result: ok. 6 passed; 0 failed
```

- [ ] **Step 3: Commit**

```bash
git add tests/api.rs
git commit -m "feat: add agent loop integration tests"
```
