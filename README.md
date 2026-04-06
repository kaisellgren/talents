# Talents

A Rust/Axum backend that uses an agentic loop to search and rank talents from a PostgreSQL database using an SGLang LLM server.

## Requirements

- Rust (stable)
- [podman](https://podman.io/) or Docker with Compose
- sqlx-cli: `cargo install sqlx-cli --no-default-features --features postgres`

## Setup

**1. Start services**

```bash
podman compose up -d
```

This starts PostgreSQL on port 5432 and SGLang on port 9000.

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
npm install
npm run dev
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

---

### Create talent

```
POST /talents
```

```bash
curl -X POST http://localhost:3000/talents \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Alice Smith",
    "skills": ["rust", "postgresql"],
    "location_city": "Helsinki",
    "location_country": "Finland",
    "available": true,
    "hourly_rate_min": 80,
    "hourly_rate_max": 120,
    "biography": "Senior Rust engineer with 5 years experience."
  }'
```

Returns `201` with the created talent including its UUID.

---

### List available talents

```
GET /talents/available
```

```bash
curl http://localhost:3000/talents/available
```

Returns all talents where `available = true`.

---

### Search talents by skills

```
GET /talents/search?skills=<comma-separated>
```

```bash
curl "http://localhost:3000/talents/search?skills=rust,postgresql"
curl "http://localhost:3000/talents/search?skills=rust&city=Helsinki&country=Finland"
```

Query parameters:
- `skills` (required) — comma-separated list, e.g. `rust,postgresql`
- `city` (optional) — filter by city
- `country` (optional) — filter by country

---

### Run agent loop

```
POST /agents/run
```

Runs a multi-step agentic loop: triage → research → constraint filter → ranking → summarizer. Retries up to 5 times with broadened keywords if no talents pass constraints.

```bash
curl -X POST http://localhost:3000/agents/run \
  -H "Content-Type: application/json" \
  -d '{"prompt": "I need a senior Rust developer in Helsinki, budget 100/hr"}'
```

Response:

```json
{
  "talents": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "Alice Smith",
      "score": 0.92,
      "reasoning": "Strong Rust skills, located in Helsinki, rate within budget.",
      "summary": "Senior Rust engineer with deep PostgreSQL experience."
    }
  ],
  "iterations": 1
}
```

- `iterations` — number of retry loops executed (max 5)
- `talents` — empty array if no matches found after all retries

## Running tests

```bash
cargo test -- --test-threads=1
```

Requires `DATABASE_URL` to point to a running Postgres instance.

## CI

GitHub Actions runs on every push and PR. See `.github/workflows/ci.yml`.
