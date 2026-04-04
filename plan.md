# React Frontend Plan

## What's already done

All backend work is complete:
- PostgreSQL schema + sqlx migrations
- Axum REST API: `POST /candidates`, `GET /candidates/available`, `GET /candidates/search`, `GET /health`, `POST /agents/run`
- Agentic loop: triage → research → constraint filter → ranking → summarizer (up to 5 retries)
- Integration tests (mock SGLang server + real Postgres)
- GitHub Actions CI
- README with API docs

---

## React Frontend Goal

A minimal employer-facing UI that lets users:
1. Submit a natural language prompt and see AI-ranked candidates returned by `POST /agents/run`
2. Browse all available candidates via `GET /candidates/available`
3. Search candidates by skills/location via `GET /candidates/search`

---

## Stack

- **Vite + React + TypeScript** — fast dev server, no CRA overhead
- **Tailwind CSS** — utility styling, no component library needed for something this simple
- **fetch API** — no axios, the endpoints are simple REST
- Served separately from the Rust backend (Vite dev server proxies `/api` to `localhost:3000`)

---

## Pages / Views

### 1. Agent Search (default view)
- Text area for the employer prompt
- Submit button → calls `POST /agents/run`
- Shows a loading spinner during the LLM call (can take several seconds)
- Renders a ranked list of candidate cards with: name, score, reasoning, summary
- Shows iteration count ("Found in N attempt(s)")

### 2. Browse Available
- Calls `GET /candidates/available` on load
- Renders candidate cards: name, skills, city/country, rate range, bio

### 3. Skill Search
- Input for skills (comma-separated) + optional city/country filters
- Calls `GET /candidates/search?skills=...&city=...&country=...`
- Renders matching candidate cards

---

## File Structure

```
frontend/
  index.html
  vite.config.ts          ← proxy /api → localhost:3000
  tailwind.config.ts
  src/
    main.tsx
    App.tsx               ← tab nav between 3 views
    api.ts                ← typed fetch wrappers for all 3 endpoints
    types.ts              ← Candidate, AgentResponse, AgentCandidate types
    components/
      CandidateCard.tsx   ← reusable card for a single candidate
      AgentSearch.tsx     ← prompt form + results
      BrowseAvailable.tsx ← available candidates list
      SkillSearch.tsx     ← skill/location filter + results
```

---

## API Types (from backend)

```typescript
// types.ts
export interface Candidate {
  id: string;
  name: string;
  skills: string[];
  location_city: string;
  location_country: string;
  role: string | null;
  available: boolean;
  hourly_rate_min: number | null;
  hourly_rate_max: number | null;
  biography: string | null;
  created_at: string;
}

export interface AgentCandidate {
  id: string;
  name: string;
  score: number;
  reasoning: string;
  summary: string;
}

export interface AgentResponse {
  candidates: AgentCandidate[];
  iterations: number;
}
```

---

## Implementation Steps

1. Scaffold `frontend/` with Vite: `npm create vite@latest frontend -- --template react-ts`
2. Install Tailwind: `npm install -D tailwindcss postcss autoprefixer && npx tailwindcss init -p`
3. Add Vite proxy in `vite.config.ts` (`/api` → `http://localhost:3000`)
4. Write `types.ts` and `api.ts` (typed fetch wrappers)
5. Build `CandidateCard` component
6. Build `AgentSearch` view (prompt form, loading state, results)
7. Build `BrowseAvailable` view
8. Build `SkillSearch` view
9. Wire up tab navigation in `App.tsx`
10. Add `frontend` dev script to README

---

## Running the Full Stack

```bash
# Terminal 1: backend
cargo run

# Terminal 2: frontend
cd frontend && npm run dev
```

Frontend at `http://localhost:5173`, backend at `http://localhost:3000`.

---

*Steps 1–9 of deployment enhancements, docs, and cleanup are complete. Only the React frontend remains.*
