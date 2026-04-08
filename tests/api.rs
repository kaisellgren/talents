mod common;

use common::{
    seed::{TalentOverrides, seed_talent},
    shared_server,
};
use serde_json::{Value, json};

// ─── POST /talents ─────────────────────────────────────────────────────────

#[tokio::test]
async fn create_talent_returns_201_with_id() {
    let srv = shared_server().await;
    let client = reqwest::Client::new();

    let res = client
        .post(format!("{}/talents", srv.app_url))
        .json(&json!({
            "name": "Kai Sellgren",
            "skills": ["rust", "axum"],
            "location_city": "Helsinki",
            "location_country": "Finland",
            "available": true,
            "hourly_rate": 100,
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

#[tokio::test]
async fn list_available_returns_only_available_talents() {
    let srv = shared_server().await;
    let client = reqwest::Client::new();

    // Ensure at least one unavailable talent exists so the filter is meaningful
    seed_talent(
        &srv.pool,
        TalentOverrides {
            available: Some(false),
            ..Default::default()
        },
    )
    .await;

    let res = client
        .get(format!("{}/talents/available", srv.app_url))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: Vec<Value> = res.json().await.unwrap();
    assert!(!body.is_empty(), "expected at least one available talent");
    assert!(
        body.iter().all(|t| t["available"] == true),
        "all returned talents should be available"
    );
}

// ─── GET /talents/search ───────────────────────────────────────────────────

#[tokio::test]
async fn search_by_skill_returns_matching_talents() {
    let srv = shared_server().await;
    let client = reqwest::Client::new();

    // Use a unique skill name to avoid collisions with other data
    let unique_skill = format!(
        "skill{}",
        &uuid::Uuid::new_v4().to_string().replace('-', "")[..8]
    );

    seed_talent(
        &srv.pool,
        TalentOverrides {
            skills: Some(vec![unique_skill.clone()]),
            ..Default::default()
        },
    )
    .await;

    let res = client
        .get(format!(
            "{}/talents/search?skills={}",
            srv.app_url, unique_skill
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: Vec<Value> = res.json().await.unwrap();
    assert!(!body.is_empty(), "expected at least one matching talent");
    assert!(
        body.iter().all(|t| t["skills"]
            .as_array()
            .unwrap()
            .contains(&json!(unique_skill))),
        "all returned talents should have the searched skill"
    );
}

#[tokio::test]
async fn search_without_skills_param_returns_400() {
    let srv = shared_server().await;
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/talents/search", srv.app_url))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
}

// ─── POST /agents/run ─────────────────────────────────────────────────────────

#[tokio::test]
async fn agent_run_returns_ranked_talents() {
    let srv = shared_server().await;
    let client = reqwest::Client::new();

    // Seed a talent that matches what the mock triage returns (skills: ["rust"])
    seed_talent(
        &srv.pool,
        TalentOverrides {
            skills: Some(vec!["rust".into()]),
            available: Some(true),
            hourly_rate: Some(100),
            ..Default::default()
        },
    )
    .await;

    let res = client
        .post(format!("{}/agents/run", srv.app_url))
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

#[tokio::test]
async fn agent_run_with_no_matching_talents_retries_and_returns_empty() {
    let srv = shared_server().await;
    let client = reqwest::Client::new();

    // Use a skill so rare it won't match any seeded talent
    let res = client
        .post(format!("{}/agents/run", srv.app_url))
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

#[tokio::test]
async fn health_returns_ok() {
    let srv = shared_server().await;
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/health", srv.app_url))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: serde_json::Value = res.json().await.unwrap();
    assert_eq!(body["status"], "ok");
}
