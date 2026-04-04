# Integration Tests — Design Spec
_Date: 2026-04-04_

## Overview

A single integration test binary (`tests/api.rs`) that exercises all HTTP endpoints against a real PostgreSQL database and a lightweight mock SGLang server. The mock server is an Axum app that inspects the system prompt keyword to determine which agent is calling and returns canned JSON matching each agent's expected response schema. Both the app under test and the mock SGLang server bind to port `0` (OS-assigned) so tests can run safely in parallel.

---

## File Structure

```
tests/
  api.rs                    ← all integration tests
  common/
    mod.rs                  ← TestContext struct, setup(), cleanup helpers
    mock_sglang.rs          ← mock SGLang Axum server
    seed.rs                 ← DB seeding helpers (insert sample candidates)
```

---

## TestContext & Setup (`common/mod.rs`)

`setup()` is called at the start of each `#[tokio::test]`:

1. Read `DATABASE_URL` from environment
2. Connect to Postgres via `sqlx::PgPool`
3. Truncate `candidates` table (clean slate)
4. Spawn mock SGLang server on port `0`, capture its address
5. Set `SGLANG_URL` environment variable to `http://127.0.0.1:<mock_port>`
6. Spawn the real Axum app on port `0`, capture its address
7. Return `TestContext { client: reqwest::Client, base_url: String, pool: PgPool }`

`TestContext` implements `Drop` (or exposes `cleanup()`) that truncates the `candidates` table after the test.

---

## Mock SGLang Server (`common/mock_sglang.rs`)

Single endpoint: `POST /v1/chat/completions`

Inspects the `messages[0].content` (system prompt) for keywords:

| Keyword | Response JSON |
|---------|---------------|
| `"triage"` | `{"required_skills":["rust"],"preferred_skills":[],"location_city":null,"location_country":null,"max_hourly_rate":null}` |
| `"ranking"` | `{"rankings":[{"candidate_id":"<uuid>","score":0.9,"reasoning":"Strong match"}]}` — UUIDs parsed from request body candidates |
| `"summarizer"` | `{"summaries":[{"candidate_id":"<uuid>","summary":"Great candidate."}]}` — UUIDs parsed from request body candidates |

For ranking and summarizer, the mock parses the candidate array from the user message in the request body and echoes back one entry per candidate UUID so the orchestrator can match them.

Response envelope (OpenAI-compatible):
```json
{
  "choices": [
    {
      "message": {
        "content": "<json string>"
      }
    }
  ]
}
```

Unrecognised system prompts return HTTP 400.

---

## Seed Helpers (`common/seed.rs`)

`seed_candidate(pool, overrides) -> Candidate` — inserts one candidate with sensible defaults, accepting field overrides. Used to set up specific test scenarios (available/unavailable, rate ranges, skill sets).

Default candidate:
- `name`: `"Test Candidate"`
- `skills`: `["rust", "postgresql"]`
- `location_city`: `"Helsinki"`, `location_country`: `"Finland"`
- `available`: `true`
- `hourly_rate_min`: `50`, `hourly_rate_max`: `100`
- `biography`: `"Experienced developer."`

---

## Test Cases (`tests/api.rs`)

| # | Test | Endpoint | Setup | Assertion |
|---|------|----------|-------|-----------|
| 1 | Create candidate | `POST /candidates` | none | 201, body has UUID |
| 2 | List available | `GET /candidates/available` | seed 1 available + 1 unavailable | returns only available |
| 3 | Search by skill | `GET /candidates/search?skills=rust` | seed rust candidate | returns 1 match |
| 4 | Search missing param | `GET /candidates/search` | none | 400 |
| 5 | Agent loop success | `POST /agents/run` | seed rust candidate | 200, candidates non-empty, iterations=1 |
| 6 | Agent loop no match | `POST /agents/run` | no candidates seeded | 200, candidates empty, iterations=5 |

---

## Environment Requirements

- `DATABASE_URL` must be set (points to a running Postgres 16 instance)
- `SGLANG_URL` is set programmatically by `setup()` — do not set it manually when running tests
- Run with: `cargo test --test api`

---

## Dependencies to Add

- `tokio` with `test` feature (already present)
- `reqwest` with `json` feature (already present)
- No new production dependencies needed; mock server reuses `axum` and `tokio` already in `Cargo.toml`
