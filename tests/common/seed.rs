use sqlx::PgPool;
use talents::db::talent::Talent;

#[derive(Default)]
pub struct TalentOverrides {
    pub name: Option<String>,
    pub skills: Option<Vec<String>>,
    pub available: Option<bool>,
    pub hourly_rate: Option<i32>,
}

/// Inserts a talent into the DB with sensible defaults, accepting field overrides.
/// Returns the inserted Talent with its DB-assigned id and created_at.
pub async fn seed_talent(pool: &PgPool, overrides: TalentOverrides) -> Talent {
    let skills = overrides
        .skills
        .unwrap_or_else(|| vec!["rust".into(), "postgresql".into()]);
    let skills_json = serde_json::to_value(&skills).unwrap();

    sqlx::query_as::<_, Talent>(
        r#"
        INSERT INTO talents (name, skills, location_city, location_country, role, available, hourly_rate, biography)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(overrides.name.unwrap_or_else(|| "Test Talent".into()))
    .bind(skills_json)
    .bind("Helsinki")
    .bind("Finland")
    .bind(Option::<String>::None)
    .bind(overrides.available.unwrap_or(true))
    .bind(overrides.hourly_rate.unwrap_or(100))
    .bind("Experienced developer.")
    .fetch_one(pool)
    .await
    .expect("Failed to seed talent")
}
