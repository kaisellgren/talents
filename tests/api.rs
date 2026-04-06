mod common;

use common::{
    seed::{TalentOverrides, seed_talent},
    setup,
};
use serde_json::{Value, json};

// ─── POST /talents ─────────────────────────────────────────────────────────

#[tokio::test(flavor = "current_thread")]
async fn create_talent_returns_201_with_id() {
    let ctx = setup().await;

    let res = ctx
        .client
        .post(format!("{}/talents", ctx.app_url))
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
    let id_str = body["id"].as_str().expect("response should contain an id");
    assert!(
        uuid::Uuid::parse_str(id_str).is_ok(),
        "id should be a valid UUID"
    );
}

// ─── GET /talents/available ────────────────────────────────────────────────

#[tokio::test(flavor = "current_thread")]
async fn list_available_returns_only_available_talents() {
    let ctx = setup().await;

    seed_talent(
        &ctx.pool,
        TalentOverrides {
            available: Some(true),
            ..Default::default()
        },
    )
    .await;
    seed_talent(
        &ctx.pool,
        TalentOverrides {
            available: Some(false),
            ..Default::default()
        },
    )
    .await;

    let res = ctx
        .client
        .get(format!("{}/talents/available", ctx.app_url))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: Vec<Value> = res.json().await.unwrap();
    assert_eq!(body.len(), 1);
    assert_eq!(body[0]["available"], true);
}

// ─── GET /talents/search ───────────────────────────────────────────────────

#[tokio::test(flavor = "current_thread")]
async fn search_by_skill_returns_matching_talents() {
    let ctx = setup().await;

    seed_talent(
        &ctx.pool,
        TalentOverrides {
            skills: Some(vec!["rust".into(), "postgresql".into()]),
            ..Default::default()
        },
    )
    .await;

    let res = ctx
        .client
        .get(format!("{}/talents/search?skills=rust", ctx.app_url))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: Vec<Value> = res.json().await.unwrap();
    assert_eq!(body.len(), 1);
    assert!(
        body[0]["skills"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("rust")),
        "returned talent should have 'rust' skill"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn search_without_skills_param_returns_400() {
    let ctx = setup().await;

    let res = ctx
        .client
        .get(format!("{}/talents/search", ctx.app_url))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
}

// ─── POST /agents/run ─────────────────────────────────────────────────────────

#[tokio::test(flavor = "current_thread")]
async fn agent_run_returns_ranked_talents() {
    let ctx = setup().await;

    // Seed a talent that matches what the mock triage returns (skills: ["rust"])
    seed_talent(
        &ctx.pool,
        TalentOverrides {
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
    let talents = body["talents"].as_array().unwrap();
    assert!(!talents.is_empty(), "expected at least one talent");
    let first = &talents[0];
    assert!(first["id"].as_str().is_some());
    assert!(first["name"].as_str().is_some());
    assert!(first["score"].as_f64().is_some());
    assert!(first["reasoning"].as_str().is_some());
    assert!(first["summary"].as_str().is_some());
}

#[tokio::test(flavor = "current_thread")]
async fn agent_run_with_no_matching_talents_retries_and_returns_empty() {
    let ctx = setup().await;
    // No talents seeded — constraint step will always return empty,
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
    // 5 is the max retry limit defined in src/agents/mod.rs (max_iterations = 5u32)
    assert_eq!(body["iterations"], 5);
    let talents = body["talents"].as_array().unwrap();
    assert!(
        talents.is_empty(),
        "expected no talents after all retries exhausted"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn health_returns_ok() {
    let ctx = setup().await;

    let res = ctx
        .client
        .get(format!("{}/health", ctx.app_url))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: serde_json::Value = res.json().await.unwrap();
    assert_eq!(body["status"], "ok");
}
