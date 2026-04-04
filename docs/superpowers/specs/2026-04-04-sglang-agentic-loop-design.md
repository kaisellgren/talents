# SGLang Agentic Loop — Design Spec
_Date: 2026-04-04_

## Overview

A controlled agentic loop implemented manually in Rust that accepts a free-form talent search prompt, runs it through a pipeline of LLM-powered agents, queries PostgreSQL for matching candidates, and returns ranked + summarized results. SGLang is the LLM backend; it never touches the database directly. All DB access is done in Rust based on structured JSON outputs from SGLang.

---

## File Structure

```
src/
  agents/
    mod.rs          ← loop orchestrator: run_agent_loop(), retry logic
    triage.rs       ← extracts skills/location/rate from prompt
    research.rs     ← queries Postgres based on triage output
    ranking.rs      ← sorts candidates by relevance to prompt
    constraint.rs   ← filters by availability, rate, required skills (pure Rust, no LLM)
    summarizer.rs   ← generates per-candidate explanation summaries
  sglang.rs         ← shared SGLang HTTP client (reqwest)
  db/
    candidate.rs    ← existing CRUD + search (unchanged)
  routes/
    candidate.rs    ← existing; /agents/run calls agents::run_agent_loop()
  main.rs
```

---

## SGLang Client (`src/sglang.rs`)

Single shared function that POSTs to SGLang's OpenAI-compatible endpoint:

- URL: `http://localhost:9000/v1/chat/completions`
- Uses `reqwest` async client
- Sets `response_format: { "type": "json_object" }` to enforce structured JSON output
- Each agent passes its own system prompt and user content
- Returns the raw JSON string; each agent deserializes into its own typed struct

---

## Agent Pipeline

### 1. Triage Agent (`triage.rs`)

**Input:** raw user prompt

**Output schema:**
```json
{
  "required_skills": ["rust", "postgresql"],
  "preferred_skills": ["docker", "axum"],
  "location_city": "Berlin",
  "location_country": "Germany",
  "max_hourly_rate": 150
}
```

All fields except `required_skills` are optional. `preferred_skills` are used only for ranking, never for filtering.

When called for **retry/broadening**, the system prompt instructs the LLM to produce fewer `required_skills` than the previous attempt (passed as context).

---

### 2. Research Agent (`research.rs`)

**Input:** triage output

**Action:** calls existing `db::candidate::search_by_skill_and_location` with `required_skills`, `location_city`, `location_country`.

**Output:** `Vec<Candidate>` from Postgres. No LLM call — pure DB query in Rust.

---

### 3. Constraint Agent (`constraint.rs`)

**Input:** `Vec<Candidate>` + triage output

**Action:** pure Rust filtering — no LLM call:
- `available = true`
- `hourly_rate_max >= triage.max_hourly_rate` (if specified)
- candidate has all `required_skills`

**Output:** filtered `Vec<Candidate>`. If empty, signals the loop to retry with broadened triage.

---

### 4. Ranking Agent (`ranking.rs`)

**Input:** filtered `Vec<Candidate>` + original prompt

**Output schema** (array, one entry per candidate):
```json
[
  { "candidate_id": "uuid", "score": 0.95, "reasoning": "Strong Rust match, Berlin location" }
]
```

Results are sorted descending by `score` in Rust after deserialization.

---

### 5. Summarizer Agent (`summarizer.rs`)

**Input:** ranked candidates + original prompt

**Output schema:**
```json
[
  { "candidate_id": "uuid", "summary": "Kai is a senior Rust engineer based in Berlin..." }
]
```

---

## Loop Orchestrator (`agents/mod.rs`)

```
pub async fn run_agent_loop(pool, prompt) -> Result<AgentResponse>

for attempt in 0..5:
    triage_output  = triage_agent(prompt, previous_keywords_if_retry)
    candidates     = research_agent(pool, triage_output)
    filtered       = constraint_agent(candidates, triage_output)
    if filtered.is_empty() && attempt < 4:
        continue  ← retry with broadening prompt
    ranked         = ranking_agent(filtered, prompt)
    summaries      = summarizer_agent(ranked_candidates, prompt)
    return AgentResponse { candidates: merged, iterations: attempt+1 }

return empty AgentResponse if all retries exhausted
```

---

## API Response

`POST /agents/run` returns:

```json
{
  "candidates": [
    {
      "id": "uuid",
      "name": "Kai Sellgren",
      "score": 0.95,
      "reasoning": "Strong Rust + location match",
      "summary": "Kai is a senior Rust engineer based in Berlin..."
    }
  ],
  "iterations": 1
}
```

---

## Error Handling

- SGLang unreachable → HTTP 502 with error message
- SGLang returns malformed JSON → HTTP 502
- All retries exhausted with no candidates → HTTP 200 with empty `candidates` array and `"iterations": 5`
- DB errors → HTTP 500

---

## Dependencies to Add

- `reqwest` with `json` feature — HTTP client for SGLang
- `serde_json` — already present via sqlx

---

## Out of Scope

- React frontend (separate step)
- Authentication
- SGLang model selection per agent
- Caching of triage/research results
