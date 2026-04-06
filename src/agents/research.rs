use anyhow::Result;
use sqlx::PgPool;

use crate::agents::triage::TriageOutput;
use crate::db::candidate::{Candidate, search_by_skills_and_location};

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
