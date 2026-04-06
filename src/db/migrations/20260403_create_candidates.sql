-- src/db/migrations/20260403_create_candidates.sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Table that holds every candidate record.
CREATE TABLE candidates (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name            TEXT NOT NULL,
    -- We store skills as a JSONB array so we can search with GIN.
    skills          JSONB NOT NULL DEFAULT '[]'::JSONB,

    -- Location fields – city and country are required, state is optional.
    location_city   TEXT NOT NULL,
    location_country TEXT NOT NULL,

    role            TEXT,           -- e.g. "Software Engineer"

    available       BOOLEAN NOT NULL DEFAULT FALSE,

    hourly_rate_min INT,
    hourly_rate_max INT,

    biography       TEXT,

    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Indexes for the most common queries.

-- Fast lookup of “available” candidates.
CREATE INDEX idx_candidates_available ON candidates (available);

-- GIN index on JSONB skills array – allows `WHERE skills @> '["Rust"]'`.
CREATE INDEX idx_candidates_skills_gin ON candidates USING gin(skills);

-- Indexes for location look‑ups (city, state, country).
CREATE INDEX idx_candidates_city   ON candidates (location_city);
CREATE INDEX idx_candidates_country ON candidates (location_country);

-- Composite index that is handy when filtering by both availability
-- and city/state/country together.
CREATE INDEX idx_candidates_avail_loc ON candidates (available, location_city, location_country);
