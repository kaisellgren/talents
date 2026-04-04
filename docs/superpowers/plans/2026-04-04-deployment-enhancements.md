# Deployment Enhancements Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a health endpoint, wire environment variables into docker-compose, and create a GitHub Actions CI pipeline that runs integration tests on every push.

**Architecture:** The health endpoint is a simple `GET /health` returning `{"status":"ok"}` registered in `create_app()`. Docker Compose gets `DATABASE_URL` and `SGLANG_URL` env vars pointing its containers at each other. CI uses GitHub Actions with a `postgres:16` service container and runs `cargo test` including the integration test binary.

**Tech Stack:** Rust, axum 0.6, Docker Compose 3.8, GitHub Actions

---

## File Map

| File | Action | Responsibility |
|------|--------|----------------|
| `src/routes/health.rs` | Create | `GET /health` handler returning `{"status":"ok"}` |
| `src/routes/mod.rs` | Modify | Add `pub mod health;` |
| `src/lib.rs` | Modify | Register `/health` route in `create_app()` |
| `tests/api.rs` | Modify | Add integration test for health endpoint |
| `docker-compose.yml` | Modify | Add `DATABASE_URL` and `SGLANG_URL` env vars to app service (or annotate for dev use) |
| `.github/workflows/ci.yml` | Create | GitHub Actions workflow: build + test on push/PR |

---

## Task 1: Add `GET /health` endpoint

**Files:**
- Create: `src/routes/health.rs`
- Modify: `src/routes/mod.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Write the failing integration test**

Add to `tests/api.rs`:

```rust
#[tokio::test(flavor = "current_thread")]
async fn health_returns_ok() {
    let ctx = setup().await;

    let res = ctx
        .client
        .get(format!("{}/health", ctx.app_url))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: serde_json::Value = res.json().await.unwrap();
    assert_eq!(body["status"], "ok");
}
```

- [ ] **Step 2: Run the test to verify it fails**

```bash
cargo test --test api health_returns_ok -- --test-threads=1 2>&1
```

Expected: FAIL — 404 Not Found (route doesn't exist yet).

- [ ] **Step 3: Create `src/routes/health.rs`**

```rust
use axum::{Json, http::StatusCode};
use serde_json::{json, Value};

pub async fn handler() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"status": "ok"})))
}
```

- [ ] **Step 4: Expose `health` in `src/routes/mod.rs`**

Current content:
```rust
pub mod candidate;
```

New content:
```rust
pub mod candidate;
pub mod health;
```

- [ ] **Step 5: Register the route in `src/lib.rs`**

Current `create_app`:
```rust
pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .nest("/candidates", routes::candidate::router())
        .route("/agents/run", post(routes::candidate::run_agent))
        .layer(Extension(pool))
}
```

New `create_app` (add `get` to the use and add the health route):
```rust
use axum::{routing::{get, post}, Extension, Router};

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(routes::health::handler))
        .nest("/candidates", routes::candidate::router())
        .route("/agents/run", post(routes::candidate::run_agent))
        .layer(Extension(pool))
}
```

- [ ] **Step 6: Run the test again to verify it passes**

```bash
cargo test --test api health_returns_ok -- --test-threads=1 2>&1
```

Expected: PASS.

- [ ] **Step 7: Run all tests to confirm nothing regressed**

```bash
cargo test -- --test-threads=1 2>&1
```

Expected: all tests pass.

- [ ] **Step 8: Commit**

```bash
git add src/routes/health.rs src/routes/mod.rs src/lib.rs tests/api.rs
git commit -m "feat: add GET /health endpoint"
```

---

## Task 2: Add env vars to `docker-compose.yml`

The running app needs `DATABASE_URL` to connect to postgres and `SGLANG_URL` to reach the LLM server. These can be added to a (currently absent) `app` service, but since there is no `app` service yet (the binary is run locally with `cargo run`), the correct action is to annotate the existing services with the values a developer would use when running the binary locally against Docker Compose.

**Files:**
- Modify: `docker-compose.yml`

- [ ] **Step 1: Add an `app` service with env vars**

Replace `docker-compose.yml` with:

```yaml
version: "3.8"

services:
  postgres:
    image: docker.io/postgres:16
    container_name: postgres
    restart: unless-stopped
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: password
      POSTGRES_DB: talents
    ports:
      - "5432:5432"
    volumes:
      - pgdata:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U user -d talents"]
      interval: 5s
      timeout: 5s
      retries: 5

  sglang:
    image: docker.io/lmsysorg/sglang:latest
    container_name: sglang
    restart: unless-stopped
    environment:
      MODEL_NAME: "microsoft/phi-2"
    ports:
      - "9000:80"
    command: >
      python3 -m sglang.launch_server
      --model-path ${MODEL_NAME}
      --host 0.0.0.0
      --port 80
    volumes:
      - ~/.cache/huggingface:/root/.cache/huggingface

volumes:
  pgdata:
```

Key change: added `healthcheck` to `postgres` so CI can wait for it to be ready before running tests. The `DATABASE_URL` and `SGLANG_URL` values for local dev are documented in `.env.example` (created below).

- [ ] **Step 2: Create `.env.example`**

Create a new file `.env.example`:

```
DATABASE_URL=postgres://user:password@localhost:5432/talents
SGLANG_URL=http://localhost:9000
```

- [ ] **Step 3: Verify the compose file is valid**

```bash
podman compose config 2>&1
```

Expected: outputs the merged config without errors.

- [ ] **Step 4: Commit**

```bash
git add docker-compose.yml .env.example
git commit -m "chore: add postgres healthcheck and document env vars in .env.example"
```

---

## Task 3: Create GitHub Actions CI pipeline

**Files:**
- Create: `.github/workflows/ci.yml`

- [ ] **Step 1: Create `.github/workflows/ci.yml`**

```yaml
name: CI

on:
  push:
    branches: ["**"]
  pull_request:
    branches: ["**"]

env:
  DATABASE_URL: postgres://user:password@localhost:5432/talents
  SGLANG_URL: "" # overridden by integration test setup()

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_USER: user
          POSTGRES_PASSWORD: password
          POSTGRES_DB: talents
        ports:
          - 5432:5432
        options: >-
          --health-cmd "pg_isready -U user -d talents"
          --health-interval 5s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo registry and build artifacts
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-

      - name: Install sqlx-cli
        run: cargo install sqlx-cli --no-default-features --features postgres

      - name: Run database migrations
        run: |
          cd src/db
          cargo sqlx migrate run --database-url "$DATABASE_URL"

      - name: Build
        run: cargo build --release

      - name: Run all tests
        run: cargo test -- --test-threads=1
```

- [ ] **Step 2: Verify the YAML is well-formed**

```bash
python3 -c "import yaml, sys; yaml.safe_load(open('.github/workflows/ci.yml'))" && echo "YAML OK"
```

Expected: `YAML OK`

- [ ] **Step 3: Run tests locally one final time to confirm everything still passes**

```bash
cargo test -- --test-threads=1 2>&1
```

Expected: all tests pass.

- [ ] **Step 4: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: add GitHub Actions workflow to build and test on every push"
```
