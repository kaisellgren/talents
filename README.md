# Talents

A demo project written in Rust that uses a controlled agentic LLM loop to search and rank talents from a PostgreSQL database.

![Screenshot](docs/screenshot.avif)

## Requirements

- Rust for backend
- podman or Docker with Compose
- sqlx-cli: `cargo install sqlx-cli --no-default-features --features postgres`
- Node.js and pnpm for frontend

## Setup

**1. Start services**

```bash
podman compose up -d
```

This starts PostgreSQL on port 5432.

**2. Configure environment**

```bash
cp .env.example .env
```

**3. Run migrations**

```bash
cd src/db
cargo sqlx migrate run --database-url "postgres://user:password@localhost:5432/talents"
cd ../..
```

**4. Start the server**

```bash
cargo run
```

Server listens on `http://127.0.0.1:3000`.

**5. Start the frontend** (optional)

```bash
cd frontend
pnpm install
pnpm dev
```

Frontend at `http://localhost:5173`. Proxies `/api` → `http://localhost:3000`.

## API

### Health

```
GET /health
```

```bash
curl http://localhost:3000/health
# {"status":"ok"}
```

## Running tests

```bash
cargo test -- --test-threads=1
```

Requires `DATABASE_URL` to point to a running Postgres instance.

## CI

GitHub Actions runs on every push and PR. See `.github/workflows/ci.yml`.
