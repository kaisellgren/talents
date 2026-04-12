# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Backend: Build & Run
- **Build the binary**: `cargo build --release`
- **Run locally**: `cargo run` – starts the Axum HTTP server on `0.0.0.0:3000` (or `$PORT`).
- **Run tests**: `cargo test`. To run a specific test: `cargo test --test <name>`.
- **Environment**: copy `.env.example` to `.env` and set `DATABASE_URL`, `LLM_URL`, `LLM_MODEL`.

## Frontend
- Always use `pnpm` instead of `npm`.
- **Dev server**: `cd frontend && pnpm dev` — proxies `/api/*` to the Rust backend at `localhost:3000`.
- **Build**: `cd frontend && pnpm build` — outputs to `frontend/dist/`.
- In production the Rust server serves `frontend/dist/` as static files; `VITE_API_BASE` is empty so requests go directly to `/agents/run`, `/talents/*` etc.

## Docker Compose
The repository ships with a `docker-compose.yml` defining:
1. **postgres** – PostgreSQL 16, exposed on port 5432.

To start: `podman compose up -d`. To stop: `podman compose down`.

### Migration
```bash
# Ensure postgres is running. Run from the repo root.
cargo sqlx migrate run --source src/db/migrations --database-url "postgres://user:password@localhost:5432/talents"
```

## Infrastructure (GCP)
- Managed via **CDKTF** (TypeScript) in `infra/`.
- Deploy infra locally: `cd infra && cdktf deploy` (requires `gcloud auth application-default login`).
- Terraform state is stored in GCS bucket `talents-493111-tfstate`.
- Cloud Run service: `talents` in `europe-west3`, scales to zero, uses Vertex AI (`gemini-2.0-flash-001`) for LLM calls via GCP metadata server auth.
- DB: Neon.tech PostgreSQL, connection string stored in Secret Manager as `database-url`.

## CI/CD (GitHub Actions)
- **`deploy.yml`**: triggers on push to `main` — runs migrations, builds Docker image, pushes to Artifact Registry, deploys to Cloud Run.
- **`deploy-infra.yml`**: triggers on push to `infra/**` or manually — runs `cdktf deploy`.
- Required GitHub secrets: `GCP_WORKLOAD_IDENTITY_PROVIDER`, `GCP_SERVICE_ACCOUNT_EMAIL`, `DATABASE_URL`.

## Project Structure
```
src/                   Rust backend
  main.rs              Entry point (binds port, connects DB)
  lib.rs               Router (API routes + static file fallback)
  agents/              Multi-agent talent matching pipeline
  db/                  SQLx queries and migrations
  routes/              Axum route handlers
  llm.rs               LLM HTTP client (Vertex AI / local)
frontend/              React + Vite SPA
  src/api.ts           API client (uses VITE_API_BASE)
infra/                 CDKTF infrastructure (TypeScript)
.github/workflows/     GitHub Actions CI/CD
Dockerfile             Multi-stage: frontend build → Rust build → runtime
```

---
**End of CLAUDE.md**
