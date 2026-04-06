use axum::http::StatusCode;
use axum::{Json, Router, routing::post};
use serde_json::{Value, json};

pub fn router() -> Router {
    Router::new().route("/v1/chat/completions", post(handle_completion))
}

async fn handle_completion(Json(body): Json<Value>) -> Result<Json<Value>, StatusCode> {
    let system_content = body["messages"]
        .as_array()
        .and_then(|msgs| msgs.iter().find(|m| m["role"] == "system"))
        .and_then(|m| m["content"].as_str())
        .unwrap_or("");

    let user_content = body["messages"]
        .as_array()
        .and_then(|msgs| msgs.iter().find(|m| m["role"] == "user"))
        .and_then(|m| m["content"].as_str())
        .unwrap_or("");

    let response_content = if system_content.contains("triage") {
        r#"{"required_skills":["rust"],"preferred_skills":[],"location_city":null,"location_country":null,"max_hourly_rate":null}"#
            .to_string()
    } else if system_content.contains("ranking") {
        let candidates = parse_candidates_from_user_content(user_content);
        if candidates.is_empty() {
            eprintln!("mock_sglang: ranking called but no candidates parsed from user content");
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
        let rankings: Vec<Value> = candidates
            .iter()
            .map(|id| json!({"candidate_id": id, "score": 0.9, "reasoning": "Strong match"}))
            .collect();
        serde_json::to_string(&json!({"rankings": rankings})).unwrap()
    } else if system_content.contains("summarizer") {
        let candidates = parse_candidates_from_user_content(user_content);
        if candidates.is_empty() {
            eprintln!("mock_sglang: summarizer called but no candidates parsed from user content");
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
        let summaries: Vec<Value> = candidates
            .iter()
            .map(|id| json!({"candidate_id": id, "summary": "Great candidate."}))
            .collect();
        serde_json::to_string(&json!({"summaries": summaries})).unwrap()
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };

    Ok(Json(json!({
        "choices": [{"message": {"content": response_content}}]
    })))
}

/// Parses candidate UUIDs from the user message content.
/// The format is: "Prompt: ...\n\nCandidates: [{"id": "uuid", ...}]"
fn parse_candidates_from_user_content(user_content: &str) -> Vec<String> {
    let candidates_part = user_content.split("Candidates: ").nth(1).unwrap_or("[]");
    let candidates: Vec<Value> = serde_json::from_str(candidates_part).unwrap_or_default();
    candidates
        .iter()
        .filter_map(|c| c["id"].as_str().map(String::from))
        .collect()
}
