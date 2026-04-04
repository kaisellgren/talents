use sqlx::PgPool;
use talents::db::candidate::Candidate;

pub struct CandidateOverrides {
    pub name: Option<String>,
    pub skills: Option<Vec<String>>,
    pub available: Option<bool>,
    pub hourly_rate_max: Option<i32>,
}

impl Default for CandidateOverrides {
    fn default() -> Self {
        Self {
            name: None,
            skills: None,
            available: None,
            hourly_rate_max: None,
        }
    }
}

/// Inserts a candidate into the DB with sensible defaults, accepting field overrides.
/// Returns the inserted Candidate with its DB-assigned id and created_at.
pub async fn seed_candidate(pool: &PgPool, overrides: CandidateOverrides) -> Candidate {
    let skills = overrides.skills.unwrap_or_else(|| vec!["rust".into(), "postgresql".into()]);
    let skills_json = serde_json::to_value(&skills).unwrap();

    sqlx::query_as::<_, Candidate>(
        r#"
        INSERT INTO candidates (name, skills, location_city, location_country, role, available, hourly_rate_min, hourly_rate_max, biography)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(overrides.name.unwrap_or_else(|| "Test Candidate".into()))
    .bind(skills_json)
    .bind("Helsinki")
    .bind("Finland")
    .bind(Option::<String>::None)
    .bind(overrides.available.unwrap_or(true))
    .bind(50_i32)
    .bind(overrides.hourly_rate_max.unwrap_or(100))
    .bind("Experienced developer.")
    .fetch_one(pool)
    .await
    .expect("Failed to seed candidate")
}
