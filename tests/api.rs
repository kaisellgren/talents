mod common;

use common::{
    seed::{seed_candidate, CandidateOverrides},
    setup,
};
use serde_json::{json, Value};

// ─── POST /candidates ─────────────────────────────────────────────────────────

#[tokio::test(flavor = "current_thread")]
async fn create_candidate_returns_201_with_id() {
    let ctx = setup().await;

    let res = ctx
        .client
        .post(format!("{}/candidates", ctx.app_url))
        .json(&json!({
            "name": "Kai Sellgren",
            "skills": ["rust", "axum"],
            "location_city": "Helsinki",
            "location_country": "Finland",
            "available": true,
            "hourly_rate_min": 80,
            "hourly_rate_max": 120,
            "biography": "Senior Rust engineer."
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 201);
    let body: Value = res.json().await.unwrap();
    assert!(body["id"].as_str().is_some(), "response should contain an id");
}

// ─── GET /candidates/available ────────────────────────────────────────────────

#[tokio::test(flavor = "current_thread")]
async fn list_available_returns_only_available_candidates() {
    let ctx = setup().await;

    seed_candidate(
        &ctx.pool,
        CandidateOverrides {
            available: Some(true),
            ..Default::default()
        },
    )
    .await;
    seed_candidate(
        &ctx.pool,
        CandidateOverrides {
            available: Some(false),
            ..Default::default()
        },
    )
    .await;

    let res = ctx
        .client
        .get(format!("{}/candidates/available", ctx.app_url))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: Vec<Value> = res.json().await.unwrap();
    assert_eq!(body.len(), 1);
    assert_eq!(body[0]["available"], true);
}

// ─── GET /candidates/search ───────────────────────────────────────────────────

#[tokio::test(flavor = "current_thread")]
async fn search_by_skill_returns_matching_candidates() {
    let ctx = setup().await;

    seed_candidate(
        &ctx.pool,
        CandidateOverrides {
            skills: Some(vec!["rust".into(), "postgresql".into()]),
            ..Default::default()
        },
    )
    .await;

    let res = ctx
        .client
        .get(format!("{}/candidates/search?skills=rust", ctx.app_url))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: Vec<Value> = res.json().await.unwrap();
    assert_eq!(body.len(), 1);
}

#[tokio::test(flavor = "current_thread")]
async fn search_without_skills_param_returns_400() {
    let ctx = setup().await;

    let res = ctx
        .client
        .get(format!("{}/candidates/search", ctx.app_url))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
}

// ─── POST /agents/run ─────────────────────────────────────────────────────────

#[tokio::test(flavor = "current_thread")]
async fn agent_run_returns_ranked_candidates() {
    let ctx = setup().await;

    // Seed a candidate that matches what the mock triage returns (skills: ["rust"])
    seed_candidate(
        &ctx.pool,
        CandidateOverrides {
            skills: Some(vec!["rust".into()]),
            available: Some(true),
            hourly_rate_max: Some(100),
            ..Default::default()
        },
    )
    .await;

    let res = ctx
        .client
        .post(format!("{}/agents/run", ctx.app_url))
        .json(&json!({"prompt": "I need a Rust developer in Helsinki"}))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: Value = res.json().await.unwrap();
    assert_eq!(body["iterations"], 1);
    let candidates = body["candidates"].as_array().unwrap();
    assert!(!candidates.is_empty(), "expected at least one candidate");
    let first = &candidates[0];
    assert!(first["id"].as_str().is_some());
    assert!(first["name"].as_str().is_some());
    assert!(first["score"].as_f64().is_some());
    assert!(first["reasoning"].as_str().is_some());
    assert!(first["summary"].as_str().is_some());
}

#[tokio::test(flavor = "current_thread")]
async fn agent_run_with_no_matching_candidates_retries_and_returns_empty() {
    let ctx = setup().await;
    // No candidates seeded — constraint step will always return empty,
    // triggering 5 retry iterations.

    let res = ctx
        .client
        .post(format!("{}/agents/run", ctx.app_url))
        .json(&json!({"prompt": "Find me a COBOL developer on the Moon"}))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: Value = res.json().await.unwrap();
    assert_eq!(body["iterations"], 5);
    let candidates = body["candidates"].as_array().unwrap();
    assert!(
        candidates.is_empty(),
        "expected no candidates after all retries exhausted"
    );
}
