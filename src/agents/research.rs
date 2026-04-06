use anyhow::Result;
use sqlx::PgPool;

use crate::agents::triage::TriageOutput;
use crate::db::talent::{Talent, search_by_skills_and_location};

/// Queries the database for talents matching the triage output.
/// No LLM call — pure DB lookup in Rust.
pub async fn run(pool: &PgPool, triage: &TriageOutput) -> Result<Vec<Talent>> {
    let talents = search_by_skills_and_location(
        pool,
        &triage.required_skills,
        triage.location_city.as_deref(),
        triage.location_country.as_deref(),
    )
    .await
    .map_err(anyhow::Error::from)?;
    Ok(talents)
}
