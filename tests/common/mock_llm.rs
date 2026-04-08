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
        let talents = parse_talents_from_user_content(user_content);
        if talents.is_empty() {
            eprintln!("mock_llm: ranking called but no talents parsed from user content");
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
        let rankings: Vec<Value> = talents
            .iter()
            .map(|id| json!({"talent_id": id, "score": 0.9, "reasoning": "Strong match"}))
            .collect();
        serde_json::to_string(&json!({"rankings": rankings})).unwrap()
    } else if system_content.contains("summarizer") {
        let talents = parse_talents_from_user_content(user_content);
        if talents.is_empty() {
            eprintln!("mock_llm: summarizer called but no talents parsed from user content");
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
        let summaries: Vec<Value> = talents
            .iter()
            .map(|id| json!({"talent_id": id, "summary": "Great talent."}))
            .collect();
        serde_json::to_string(&json!({"summaries": summaries})).unwrap()
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };

    Ok(Json(json!({
        "choices": [{"message": {"content": response_content}}]
    })))
}

/// Parses talent UUIDs from the user message content.
/// The format is: "Prompt: ...\n\nTalents: [{"id": "uuid", ...}]"
fn parse_talents_from_user_content(user_content: &str) -> Vec<String> {
    let talents_part = user_content.split("Talents: ").nth(1).unwrap_or("[]");
    let talents: Vec<Value> = serde_json::from_str(talents_part).unwrap_or_default();
    talents
        .iter()
        .filter_map(|c| c["id"].as_str().map(String::from))
        .collect()
}
