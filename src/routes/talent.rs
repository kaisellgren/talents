use axum::{
    Json, Router,
    extract::{Extension, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::talent as db_talent;
use db_talent::Talent;

/// Query parameters for paginated listing.
#[derive(Debug, Deserialize)]
struct PaginationParams {
    limit: Option<i64>,
    offset: Option<i64>,
}

/// Query parameters for talent search.
#[derive(Debug, Deserialize)]
struct SearchParams {
    /// Comma-separated list of skills, e.g. `?skills=rust,postgresql`
    skills: Option<String>,
    city: Option<String>,
    country: Option<String>,
}

/// Request body for creating a new talent.
#[derive(Debug, Deserialize)]
pub struct NewTalent {
    pub name: String,
    pub skills: Vec<String>,
    pub location_city: String,
    pub location_country: String,
    pub role: Option<String>,
    pub available: bool,
    pub hourly_rate: i32,
    pub biography: Option<String>,
}

/// Simple agent request payload.
#[derive(Debug, Deserialize)]
pub struct AgentRequest {
    pub prompt: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

fn api_error_response(err: anyhow::Error) -> Response {
    let status = if err.root_cause().is::<sqlx::Error>() {
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::BAD_REQUEST
    };
    (
        status,
        Json(ErrorResponse {
            error: err.to_string(),
        }),
    )
        .into_response()
}

pub fn router() -> Router {
    Router::new()
        .route("/", post(create_talent))
        .route("/available", get(list_available))
        .route("/search", get(search_talents))
}

async fn create_talent(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<NewTalent>,
) -> Result<(StatusCode, Json<Talent>), Response> {
    let talent = Talent {
        id: Uuid::new_v4(), // placeholder; real insert will generate
        name: body.name,
        skills: body.skills,
        location_city: body.location_city,
        location_country: body.location_country,
        role: body.role,
        available: body.available,
        hourly_rate: body.hourly_rate,
        biography: body.biography,
        created_at: chrono::Utc::now(),
    };

    let inserted = db_talent::create_talent(&pool, talent)
        .await
        .map_err(|e| api_error_response(anyhow::Error::from(e)))?;
    Ok((StatusCode::CREATED, Json(inserted)))
}

async fn list_available(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<Talent>>, Response> {
    let limit = params.limit.unwrap_or(30).min(100).max(1);
    let offset = params.offset.unwrap_or(0).max(0);
    let talents = db_talent::list_available(&pool, limit, offset)
        .await
        .map_err(|e| api_error_response(anyhow::Error::from(e)))?;
    Ok(Json(talents))
}

async fn search_talents(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<Talent>>, Response> {
    let skills: Vec<String> = match params.skills.as_deref() {
        None | Some("") => {
            return Err(api_error_response(anyhow::anyhow!(
                "'skills' query param required"
            )));
        }
        Some(s) => s.split(',').map(|s| s.trim().to_string()).collect(),
    };

    let talents = db_talent::search_by_skills_and_location(
        &pool,
        &skills,
        params.city.as_deref(),
        params.country.as_deref(),
    )
    .await
    .map_err(|e| api_error_response(anyhow::Error::from(e)))?;
    Ok(Json(talents))
}

pub async fn run_agent(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<AgentRequest>,
) -> Result<Json<crate::agents::AgentResponse>, Response> {
    let response = crate::agents::run_agent_loop(&pool, &body.prompt)
        .await
        .map_err(api_error_response)?;
    Ok(Json(response))
}
