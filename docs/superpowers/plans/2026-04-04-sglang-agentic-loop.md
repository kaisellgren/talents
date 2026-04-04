# SGLang Agentic Loop Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a controlled agentic loop in Rust that accepts a free-form talent search prompt and returns ranked, summarized candidates by coordinating 5 LLM-powered agents against PostgreSQL.

**Architecture:** The `/agents/run` HTTP endpoint delegates to `agents::run_agent_loop()`, which runs triage → research → constraint → ranking → summarizer in sequence, retrying up to 5 times with broadened keywords if the constraint step produces no candidates. SGLang is called via a shared HTTP client (`src/sglang.rs`) using its OpenAI-compatible endpoint with structured JSON output. All DB access happens in Rust based on LLM-provided keywords.

**Tech Stack:** Rust, Axum, sqlx (Postgres), reqwest (SGLang HTTP client), serde_json, anyhow, tokio

---

## File Map

| File | Action | Responsibility |
|------|--------|----------------|
| `Cargo.toml` | Modify | Add `reqwest` dependency |
| `src/sglang.rs` | Create | Shared SGLang HTTP client |
| `src/agents/mod.rs` | Create | Loop orchestrator, shared output types, `AgentResponse` |
| `src/agents/triage.rs` | Create | Extract skills/location/rate from prompt |
| `src/agents/research.rs` | Create | Query Postgres using triage output |
| `src/agents/constraint.rs` | Create | Pure Rust: filter by availability, rate, required skills |
| `src/agents/ranking.rs` | Create | LLM ranks candidates by relevance to prompt |
| `src/agents/summarizer.rs` | Create | LLM generates per-candidate summaries |
| `src/db/candidate.rs` | Modify | Update `search_by_skill_and_location` to accept multiple skills |
| `src/routes/candidate.rs` | Modify | Wire `run_agent` handler to call `agents::run_agent_loop()` |
| `src/main.rs` | Modify | Declare `mod agents` |
| `tests/agents_integration.rs` | Create | Integration test for constraint agent logic |

---

## Task 1: Add `reqwest` to Cargo.toml

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Add reqwest dependency**

In `Cargo.toml`, add under `[dependencies]`:

```toml
reqwest = { version = "0.11", features = ["json"] }
```

- [ ] **Step 2: Verify it compiles**

```bash
cargo check
```

Expected: no errors.

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: add reqwest dependency for SGLang HTTP client"
```

---

## Task 2: Create SGLang HTTP client (`src/sglang.rs`)

**Files:**
- Create: `src/sglang.rs`
- Modify: `src/main.rs` (add `mod sglang;`)

- [ ] **Step 1: Create `src/sglang.rs`**

```rust
use anyhow::{Context, Result};
use serde_json::{json, Value};

/// Sends a chat completion request to the SGLang server.
/// Returns the raw content string from the first choice.
pub async fn chat_completion(system_prompt: &str, user_content: &str) -> Result<String> {
    let sglang_url = std::env::var("SGLANG_URL")
        .unwrap_or_else(|_| "http://localhost:9000".to_string());

    let client = reqwest::Client::new();
    let body = json!({
        "model": "default",
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_content}
        ],
        "response_format": {"type": "json_object"}
    });

    let response = client
        .post(format!("{}/v1/chat/completions", sglang_url))
        .json(&body)
        .send()
        .await
        .context("Failed to reach SGLang server")?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("SGLang returned {}: {}", status, text);
    }

    let json: Value = response.json().await.context("Failed to parse SGLang response")?;
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .context("Missing content in SGLang response")?
        .to_string();

    Ok(content)
}
```

- [ ] **Step 2: Declare module in `src/main.rs`**

Add `mod sglang;` after the existing `mod db;` line:

```rust
mod db;
mod sglang;
mod routes {
    pub mod candidate;
}
```

- [ ] **Step 3: Verify compilation**

```bash
cargo check
```

Expected: no errors.

- [ ] **Step 4: Commit**

```bash
git add src/sglang.rs src/main.rs
git commit -m "feat: add SGLang HTTP client"
```

---

## Task 3: Update DB search to accept multiple skills

The existing `search_by_skill_and_location` only takes one skill string. The research agent needs to filter by multiple required skills and use parameterized queries for locations (currently uses string formatting which risks SQL injection).

**Files:**
- Modify: `src/db/candidate.rs`

- [ ] **Step 1: Replace `search_by_skill_and_location` in `src/db/candidate.rs`**

Replace the existing function (lines 56–83) with:

```rust
/// Search candidates matching all required skills and optional location filters.
/// Uses parameterized queries throughout to prevent SQL injection.
pub async fn search_by_skills_and_location(
    pool: &PgPool,
    required_skills: &[String],
    city: Option<&str>,
    country: Option<&str>,
) -> Result<Vec<Candidate>, sqlx::Error> {
    let skills_lower: Vec<String> = required_skills
        .iter()
        .map(|s| s.to_ascii_lowercase())
        .collect();
    let skills_json = serde_json::to_value(&skills_lower).unwrap();

    // Build query with optional location conditions using $2/$3 parameters
    let query_str = match (city, country) {
        (Some(_), Some(_)) => {
            "SELECT * FROM candidates WHERE skills @> $1::jsonb AND location_city = $2 AND location_country = $3"
        }
        (Some(_), None) => {
            "SELECT * FROM candidates WHERE skills @> $1::jsonb AND location_city = $2"
        }
        (None, Some(_)) => {
            "SELECT * FROM candidates WHERE skills @> $1::jsonb AND location_country = $2"
        }
        (None, None) => "SELECT * FROM candidates WHERE skills @> $1::jsonb",
    };

    let mut q = query_as::<_, Candidate>(query_str).bind(skills_json);
    if let Some(c) = city {
        q = q.bind(c);
    } else if let Some(co) = country {
        q = q.bind(co);
    }

    q.fetch_all(pool).await
}
```

- [ ] **Step 2: Update the caller in `src/routes/candidate.rs`**

The existing `search_candidates` handler calls the old function name. Update it:

```rust
async fn search_candidates(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<Candidate>>, Response> {
    let skill = params.get("skills").cloned();
    let city = params.get("city").cloned();
    let country = params.get("country").cloned();

    if skill.is_none() {
        return Err(api_error_response(anyhow::anyhow!(
            "'skills' query param required"
        )));
    }

    let skills = vec![skill.unwrap()];
    let candidates = db_candidate::search_by_skills_and_location(
        &pool,
        &skills,
        city.as_deref(),
        country.as_deref(),
    )
    .await
    .map_err(|e| api_error_response(anyhow::Error::from(e)))?;
    Ok(Json(candidates))
}
```

- [ ] **Step 3: Verify compilation**

```bash
cargo check
```

Expected: no errors.

- [ ] **Step 4: Commit**

```bash
git add src/db/candidate.rs src/routes/candidate.rs
git commit -m "fix: update search to accept multiple skills with parameterized location queries"
```

---

## Task 4: Create triage agent (`src/agents/triage.rs`)

**Files:**
- Create: `src/agents/triage.rs`

- [ ] **Step 1: Create `src/agents/triage.rs`**

```rust
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TriageOutput {
    pub required_skills: Vec<String>,
    #[serde(default)]
    pub preferred_skills: Vec<String>,
    pub location_city: Option<String>,
    pub location_country: Option<String>,
    pub max_hourly_rate: Option<i32>,
}

/// Calls the triage agent to extract structured search criteria from a prompt.
/// Pass `previous_required_skills` on retry to instruct the LLM to broaden the search.
pub async fn run(prompt: &str, previous_required_skills: Option<&[String]>) -> Result<TriageOutput> {
    let system_prompt = if let Some(prev) = previous_required_skills {
        format!(
            "You are a talent search triage assistant. Extract search criteria from the prompt as JSON.\n\
            A previous search with required_skills {:?} returned no results. \
            Produce FEWER required_skills (broaden the search) while keeping the most important ones.\n\
            Output only JSON with keys: required_skills (array), preferred_skills (array), \
            location_city (string or null), location_country (string or null), max_hourly_rate (number or null).",
            prev
        )
    } else {
        "You are a talent search triage assistant. Extract search criteria from the prompt as JSON.\n\
        Output only JSON with keys: required_skills (array of lowercase skill strings), \
        preferred_skills (array of lowercase skill strings, used for ranking only), \
        location_city (string or null), location_country (string or null), \
        max_hourly_rate (number or null).".to_string()
    };

    let content = crate::sglang::chat_completion(&system_prompt, prompt).await?;
    let output: TriageOutput = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Triage agent returned invalid JSON: {}: {}", e, content))?;
    Ok(output)
}
```

---

## Task 5: Create research agent (`src/agents/research.rs`)

**Files:**
- Create: `src/agents/research.rs`

- [ ] **Step 1: Create `src/agents/research.rs`**

```rust
use anyhow::Result;
use sqlx::PgPool;

use crate::agents::triage::TriageOutput;
use crate::db::candidate::{search_by_skills_and_location, Candidate};

/// Queries the database for candidates matching the triage output.
/// No LLM call — pure DB lookup in Rust.
pub async fn run(pool: &PgPool, triage: &TriageOutput) -> Result<Vec<Candidate>> {
    let candidates = search_by_skills_and_location(
        pool,
        &triage.required_skills,
        triage.location_city.as_deref(),
        triage.location_country.as_deref(),
    )
    .await
    .map_err(anyhow::Error::from)?;
    Ok(candidates)
}
```

---

## Task 6: Create constraint agent (`src/agents/constraint.rs`)

This agent is pure Rust — no LLM call. It is the most testable component.

**Files:**
- Create: `src/agents/constraint.rs`
- Modify: `tests/agents_integration.rs` (create)

- [ ] **Step 1: Write the failing test first**

Create `tests/agents_integration.rs`:

```rust
use rust_agent_demo::agents::constraint;
use rust_agent_demo::agents::triage::TriageOutput;
use rust_agent_demo::db::candidate::Candidate;
use chrono::Utc;
use uuid::Uuid;

fn make_candidate(
    skills: Vec<String>,
    available: bool,
    hourly_rate_max: Option<i32>,
) -> Candidate {
    Candidate {
        id: Uuid::new_v4(),
        name: "Test".to_string(),
        skills,
        location_city: "Helsinki".to_string(),
        location_country: "Finland".to_string(),
        role: None,
        available,
        hourly_rate_min: None,
        hourly_rate_max,
        biography: None,
        created_at: Utc::now(),
    }
}

fn make_triage(required: Vec<&str>, max_rate: Option<i32>) -> TriageOutput {
    TriageOutput {
        required_skills: required.into_iter().map(String::from).collect(),
        preferred_skills: vec![],
        location_city: None,
        location_country: None,
        max_hourly_rate: max_rate,
    }
}

#[test]
fn keeps_available_candidate_matching_skills_and_rate() {
    let candidates = vec![make_candidate(vec!["rust".into()], true, Some(100))];
    let triage = make_triage(vec!["rust"], Some(100));
    let result = constraint::run(candidates, &triage);
    assert_eq!(result.len(), 1);
}

#[test]
fn removes_unavailable_candidate() {
    let candidates = vec![make_candidate(vec!["rust".into()], false, Some(100))];
    let triage = make_triage(vec!["rust"], None);
    let result = constraint::run(candidates, &triage);
    assert!(result.is_empty());
}

#[test]
fn removes_candidate_exceeding_max_rate() {
    let candidates = vec![make_candidate(vec!["rust".into()], true, Some(200))];
    let triage = make_triage(vec!["rust"], Some(100));
    let result = constraint::run(candidates, &triage);
    assert!(result.is_empty());
}

#[test]
fn removes_candidate_missing_required_skill() {
    let candidates = vec![make_candidate(vec!["python".into()], true, Some(80))];
    let triage = make_triage(vec!["rust"], None);
    let result = constraint::run(candidates, &triage);
    assert!(result.is_empty());
}

#[test]
fn keeps_candidate_when_no_rate_limit_specified() {
    let candidates = vec![make_candidate(vec!["rust".into()], true, None)];
    let triage = make_triage(vec!["rust"], None);
    let result = constraint::run(candidates, &triage);
    assert_eq!(result.len(), 1);
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cargo test --test agents_integration 2>&1 | head -20
```

Expected: compile error — `constraint` module and `run` function not found.

- [ ] **Step 3: Create `src/agents/constraint.rs`**

```rust
use crate::agents::triage::TriageOutput;
use crate::db::candidate::Candidate;

/// Filters candidates using pure Rust logic — no LLM call.
/// Removes candidates that are unavailable, exceed the max hourly rate,
/// or are missing any required skill.
pub fn run(candidates: Vec<Candidate>, triage: &TriageOutput) -> Vec<Candidate> {
    candidates
        .into_iter()
        .filter(|c| {
            if !c.available {
                return false;
            }
            if let Some(max_rate) = triage.max_hourly_rate {
                if let Some(candidate_max) = c.hourly_rate_max {
                    if candidate_max > max_rate {
                        return false;
                    }
                }
            }
            let skills_lower: Vec<String> =
                c.skills.iter().map(|s| s.to_ascii_lowercase()).collect();
            triage
                .required_skills
                .iter()
                .all(|req| skills_lower.contains(&req.to_ascii_lowercase()))
        })
        .collect()
}
```

- [ ] **Step 4: Add lib.rs so integration tests can import modules**

Create `src/lib.rs`:

```rust
pub mod agents;
pub mod db;
pub mod sglang;
```

- [ ] **Step 5: Declare agents module in `src/main.rs`**

```rust
mod agents;
mod db;
mod sglang;
mod routes {
    pub mod candidate;
}
```

- [ ] **Step 6: Create `src/agents/mod.rs` (stub for now — will be filled in Task 9)**

```rust
pub mod constraint;
pub mod research;
pub mod triage;
pub mod ranking;
pub mod summarizer;
```

- [ ] **Step 7: Run tests to verify they pass**

```bash
cargo test --test agents_integration
```

Expected:
```
test keeps_available_candidate_matching_skills_and_rate ... ok
test removes_unavailable_candidate ... ok
test removes_candidate_exceeding_max_rate ... ok
test removes_candidate_missing_required_skill ... ok
test keeps_candidate_when_no_rate_limit_specified ... ok
```

- [ ] **Step 8: Commit**

```bash
git add src/agents/ src/lib.rs src/main.rs tests/agents_integration.rs
git commit -m "feat: add constraint agent with integration tests"
```

---

## Task 7: Create ranking agent (`src/agents/ranking.rs`)

**Files:**
- Create: `src/agents/ranking.rs`

- [ ] **Step 1: Create `src/agents/ranking.rs`**

```rust
use anyhow::Result;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::candidate::Candidate;

#[derive(Debug, Clone, Deserialize)]
pub struct RankedCandidate {
    pub candidate_id: Uuid,
    pub score: f64,
    pub reasoning: String,
}

/// Asks the LLM to rank candidates by relevance to the original prompt.
/// Returns candidates sorted descending by score.
pub async fn run(candidates: &[Candidate], prompt: &str) -> Result<Vec<RankedCandidate>> {
    let system_prompt = "You are a talent ranking assistant. \
        Given a list of candidates and a search prompt, rank the candidates by relevance. \
        Consider skills match, location preference, and cost preference as expressed in the prompt. \
        Output JSON: {\"rankings\": [{\"candidate_id\": \"<uuid>\", \"score\": <0.0-1.0>, \"reasoning\": \"<brief reason>\"}]}";

    let candidates_json = serde_json::to_string(candidates)?;
    let user_content = format!("Prompt: {}\n\nCandidates: {}", prompt, candidates_json);

    let content = crate::sglang::chat_completion(system_prompt, &user_content).await?;

    #[derive(Deserialize)]
    struct RankingResponse {
        rankings: Vec<RankedCandidate>,
    }

    let response: RankingResponse = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Ranking agent returned invalid JSON: {}: {}", e, content))?;

    let mut rankings = response.rankings;
    rankings.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    Ok(rankings)
}
```

---

## Task 8: Create summarizer agent (`src/agents/summarizer.rs`)

**Files:**
- Create: `src/agents/summarizer.rs`

- [ ] **Step 1: Create `src/agents/summarizer.rs`**

```rust
use anyhow::Result;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::candidate::Candidate;

#[derive(Debug, Clone, Deserialize)]
pub struct SummarizedCandidate {
    pub candidate_id: Uuid,
    pub summary: String,
}

/// Asks the LLM to generate a short per-candidate summary explaining
/// why each candidate suits the original prompt.
pub async fn run(candidates: &[Candidate], prompt: &str) -> Result<Vec<SummarizedCandidate>> {
    let system_prompt = "You are a talent summarizer. \
        For each candidate, write a 2-3 sentence summary explaining why they are well-suited \
        for the given search prompt. Be specific about their skills, location, and rate. \
        Output JSON: {\"summaries\": [{\"candidate_id\": \"<uuid>\", \"summary\": \"<text>\"}]}";

    let candidates_json = serde_json::to_string(candidates)?;
    let user_content = format!("Prompt: {}\n\nCandidates: {}", prompt, candidates_json);

    let content = crate::sglang::chat_completion(system_prompt, &user_content).await?;

    #[derive(Deserialize)]
    struct SummaryResponse {
        summaries: Vec<SummarizedCandidate>,
    }

    let response: SummaryResponse = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Summarizer agent returned invalid JSON: {}: {}", e, content))?;

    Ok(response.summaries)
}
```

---

## Task 9: Implement loop orchestrator (`src/agents/mod.rs`)

**Files:**
- Modify: `src/agents/mod.rs`

- [ ] **Step 1: Replace stub `src/agents/mod.rs` with the full orchestrator**

```rust
pub mod constraint;
pub mod ranking;
pub mod research;
pub mod summarizer;
pub mod triage;

use anyhow::Result;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct AgentCandidate {
    pub id: Uuid,
    pub name: String,
    pub score: f64,
    pub reasoning: String,
    pub summary: String,
}

#[derive(Debug, Serialize)]
pub struct AgentResponse {
    pub candidates: Vec<AgentCandidate>,
    pub iterations: u32,
}

/// Runs the full agentic loop: triage → research → constraint → ranking → summarizer.
/// Retries up to 5 times with broadened keywords if constraint produces no candidates.
pub async fn run_agent_loop(pool: &PgPool, prompt: &str) -> Result<AgentResponse> {
    let max_iterations = 5u32;
    let mut previous_required_skills: Option<Vec<String>> = None;

    for iteration in 1..=max_iterations {
        let triage = triage::run(prompt, previous_required_skills.as_deref()).await?;
        let candidates = research::run(pool, &triage).await?;
        let filtered = constraint::run(candidates, &triage);

        if filtered.is_empty() {
            if iteration == max_iterations {
                return Ok(AgentResponse {
                    candidates: vec![],
                    iterations: iteration,
                });
            }
            previous_required_skills = Some(triage.required_skills.clone());
            continue;
        }

        let rankings = ranking::run(&filtered, prompt).await?;
        let summaries = summarizer::run(&filtered, prompt).await?;

        let candidates = rankings
            .into_iter()
            .filter_map(|r| {
                let candidate = filtered.iter().find(|c| c.id == r.candidate_id)?;
                let summary = summaries
                    .iter()
                    .find(|s| s.candidate_id == r.candidate_id)
                    .map(|s| s.summary.clone())
                    .unwrap_or_default();
                Some(AgentCandidate {
                    id: candidate.id,
                    name: candidate.name.clone(),
                    score: r.score,
                    reasoning: r.reasoning,
                    summary,
                })
            })
            .collect();

        return Ok(AgentResponse {
            candidates,
            iterations: iteration,
        });
    }

    Ok(AgentResponse {
        candidates: vec![],
        iterations: max_iterations,
    })
}
```

- [ ] **Step 2: Verify compilation**

```bash
cargo check
```

Expected: no errors.

- [ ] **Step 3: Commit**

```bash
git add src/agents/mod.rs src/agents/triage.rs src/agents/research.rs src/agents/ranking.rs src/agents/summarizer.rs
git commit -m "feat: implement agentic loop orchestrator and all agent modules"
```

---

## Task 10: Wire `/agents/run` route to the loop

**Files:**
- Modify: `src/routes/candidate.rs`

- [ ] **Step 1: Update the `run_agent` handler in `src/routes/candidate.rs`**

Replace the placeholder `run_agent` function (currently lines 120–127):

```rust
pub async fn run_agent(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<AgentRequest>,
) -> Result<Json<serde_json::Value>, Response> {
    let response = crate::agents::run_agent_loop(&pool, &body.prompt)
        .await
        .map_err(|e| api_error_response(e))?;
    Ok(Json(serde_json::to_value(response).unwrap()))
}
```

- [ ] **Step 2: Verify compilation**

```bash
cargo check
```

Expected: no errors.

- [ ] **Step 3: Run all tests**

```bash
cargo test
```

Expected: all tests pass.

- [ ] **Step 4: Commit**

```bash
git add src/routes/candidate.rs
git commit -m "feat: wire /agents/run endpoint to agentic loop"
```

---

## Task 11: Smoke test the full loop manually

- [ ] **Step 1: Start services**

```bash
podman compose up -d
```

Expected: postgres and sglang containers running.

- [ ] **Step 2: Run migrations**

```bash
cd src/db && cargo sqlx migrate run --database-url "postgres://user:password@localhost:5432/talents"
cd ../..
```

- [ ] **Step 3: Start the server**

```bash
cargo run
```

Expected: `Listening on 127.0.0.1:3000`

- [ ] **Step 4: Seed a candidate**

```bash
curl -s -X POST http://localhost:3000/candidates \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Kai Sellgren",
    "skills": ["rust", "postgresql", "axum"],
    "location_city": "Helsinki",
    "location_country": "Finland",
    "role": "Backend Engineer",
    "available": true,
    "hourly_rate_min": 80,
    "hourly_rate_max": 120,
    "biography": "Senior Rust engineer with 5 years experience."
  }' | jq .
```

Expected: candidate JSON with a UUID.

- [ ] **Step 5: Call the agent endpoint**

```bash
curl -s -X POST http://localhost:3000/agents/run \
  -H "Content-Type: application/json" \
  -d '{"prompt": "I need a Rust backend developer in Finland, budget up to 130 per hour"}' | jq .
```

Expected: JSON with `candidates` array and `iterations` field.

- [ ] **Step 6: Commit if any adjustments were needed**

```bash
git add -p
git commit -m "fix: adjust agent prompts/parsing based on smoke test"
```
