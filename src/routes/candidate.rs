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

use crate::db::candidate as db_candidate;
use db_candidate::Candidate;

/// Query parameters for candidate search.
#[derive(Debug, Deserialize)]
struct SearchParams {
    #[serde(default)]
    skills: Vec<String>,
    city: Option<String>,
    country: Option<String>,
}

/// Request body for creating a new candidate.
#[derive(Debug, Deserialize)]
pub struct NewCandidate {
    pub name: String,
    pub skills: Vec<String>,
    pub location_city: String,
    pub location_country: String,
    pub role: Option<String>,
    pub available: bool,
    pub hourly_rate_min: Option<i32>,
    pub hourly_rate_max: Option<i32>,
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
        .route("/", post(create_candidate))
        .route("/available", get(list_available))
        .route("/search", get(search_candidates))
        .route("/agents/run", post(run_agent))
}

async fn create_candidate(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<NewCandidate>,
) -> Result<(StatusCode, Json<Candidate>), Response> {
    let candidate = Candidate {
        id: Uuid::new_v4(), // placeholder; real insert will generate
        name: body.name,
        skills: body.skills,
        location_city: body.location_city,
        location_country: body.location_country,
        role: body.role,
        available: body.available,
        hourly_rate_min: body.hourly_rate_min,
        hourly_rate_max: body.hourly_rate_max,
        biography: body.biography,
        created_at: chrono::Utc::now(),
    };

    let inserted = db_candidate::create_candidate(&pool, candidate)
        .await
        .map_err(|e| api_error_response(anyhow::Error::from(e)))?;
    Ok((StatusCode::CREATED, Json(inserted)))
}

async fn list_available(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Candidate>>, Response> {
    let candidates = db_candidate::list_available(&pool)
        .await
        .map_err(|e| api_error_response(anyhow::Error::from(e)))?;
    Ok(Json(candidates))
}

async fn search_candidates(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<Candidate>>, Response> {
    if params.skills.is_empty() {
        return Err(api_error_response(anyhow::anyhow!(
            "'skills' query param required"
        )));
    }

    let candidates = db_candidate::search_by_skills_and_location(
        &pool,
        &params.skills,
        params.city.as_deref(),
        params.country.as_deref(),
    )
    .await
    .map_err(|e| api_error_response(anyhow::Error::from(e)))?;
    Ok(Json(candidates))
}

async fn run_agent(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<AgentRequest>,
) -> Result<Json<crate::agents::AgentResponse>, Response> {
    let response = crate::agents::run_agent_loop(&pool, &body.prompt)
        .await
        .map_err(api_error_response)?;
    Ok(Json(response))
}
