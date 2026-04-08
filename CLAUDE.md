# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run
- **Build the binary**: `cargo build --release`
- **Run locally**: `cargo run` – starts a minimal Rust program that prints *Hello, world!*.
- **Run tests** (none currently): `cargo test`. To run a specific test file or module, use `cargo test --test <name>`.

## Docker Compose
The repository ships with a `docker-compose.yml` defining two services:
1. **postgres** – PostgreSQL 16, exposed on port 5432.

To start both containers: `podman compose up -d`. To stop them: `podman compose down`.

### Migration
Database migrations are located under `src/db/migrations`. Apply the migration by running:
```bash
# Ensure postgres service is running first. Ensure you are at directly 'src/db'.
cargo sqlx migrate run --database-url "postgres://user:password@localhost:5432/talents"
```

## Project Structure (High‑Level)
- `Cargo.toml`: Rust package metadata; currently no external dependencies.
- `src/main.rs`: Application entry point – placeholder for future logic.
- `src/db/migrations/`: Folder containing raw SQL migration files.
- `docker-compose.yml`: Service definitions for PostgreSQL.

## Development Notes
- The repository is intentionally minimal; additional crates (e.g., `sqlx`, `serde`) will be added as the application grows.
- When adding new modules, follow Rust's module system conventions: create a file in `src/` or sub‑folder and expose public items via `pub mod`.
- Tests should live under `tests/` or use integration tests in `src/tests`. Use `cargo test` to run them.
- Keep the Docker Compose configuration up‑to‑date with service versions.

---
**End of CLAUDE.md**
