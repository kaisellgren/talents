use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, query_as};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
pub struct Talent {
    pub id: Uuid,
    pub name: String,
    #[sqlx(json)]
    pub skills: Vec<String>,
    pub location_city: String,
    pub location_country: String,
    pub role: Option<String>,
    pub available: bool,
    pub hourly_rate: i32,
    pub biography: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub async fn create_talent(
    pool: &PgPool,
    talent: Talent,
) -> Result<Talent, sqlx::Error> {
    let skills_json =
        serde_json::to_value(&talent.skills).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    let rec = query_as::<_, Talent>(
        r#"
        INSERT INTO talents (
            name, skills, location_city, location_country,
            role, available, hourly_rate, biography
        ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
        RETURNING *
        "#,
    )
    .bind(talent.name)
    .bind(skills_json)
    .bind(talent.location_city)
    .bind(talent.location_country)
    .bind(talent.role)
    .bind(talent.available)
    .bind(talent.hourly_rate)
    .bind(talent.biography)
    .fetch_one(pool)
    .await?;
    Ok(rec)
}

/// Retrieve available talents with pagination.
pub async fn list_available(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Talent>, sqlx::Error> {
    let rows = query_as::<_, Talent>(
        "SELECT * FROM talents WHERE available = true ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Search talents matching all required skills and optional location filters.
/// Uses parameterized queries throughout to prevent SQL injection.
pub async fn search_by_skills_and_location(
    pool: &PgPool,
    required_skills: &[String],
    city: Option<&str>,
    country: Option<&str>,
) -> Result<Vec<Talent>, sqlx::Error> {
    if required_skills.is_empty() {
        return Ok(vec![]);
    }

    let skills_lower: Vec<String> = required_skills
        .iter()
        .map(|s| s.to_ascii_lowercase())
        .collect();
    let skills_json =
        serde_json::to_value(&skills_lower).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

    // Build query with optional location conditions using $2/$3 parameters
    let query_str = match (city, country) {
        (Some(_), Some(_)) => {
            "SELECT * FROM talents WHERE skills @> $1::jsonb AND location_city = $2 AND location_country = $3"
        }
        (Some(_), None) => {
            "SELECT * FROM talents WHERE skills @> $1::jsonb AND location_city = $2"
        }
        (None, Some(_)) => {
            "SELECT * FROM talents WHERE skills @> $1::jsonb AND location_country = $2"
        }
        (None, None) => "SELECT * FROM talents WHERE skills @> $1::jsonb",
    };

    let mut q = query_as::<_, Talent>(query_str).bind(skills_json);
    if let Some(c) = city {
        q = q.bind(c);
    }
    if let Some(co) = country {
        q = q.bind(co);
    }

    q.fetch_all(pool).await
}
