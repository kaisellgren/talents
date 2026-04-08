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
        // Return a skill that matches the user's prompt so constraint tests work correctly.
        // For prompts asking for impossible skills (COBOL on the Moon), return a skill
        // that won't match any seeded talent so the retry loop exhausts its iterations.
        let skill = if user_content.to_lowercase().contains("cobol") {
            "cobol_moon_nomatch_xyz"
        } else {
            "rust"
        };
        format!(
            r#"{{"required_skills":["{}"],"preferred_skills":[],"location_city":null,"location_country":null,"max_hourly_rate":null}}"#,
            skill
        )
    } else if system_content.contains("ranking") {
        let count = parse_talent_count_from_user_content(user_content);
        if count == 0 {
            eprintln!("mock_llm: ranking called but no talents parsed from user content");
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
        let rankings: Vec<Value> = (0..count)
            .map(|i| json!({"talent_index": i, "score": 0.9, "reasoning": "Strong match"}))
            .collect();
        serde_json::to_string(&json!({"rankings": rankings})).unwrap()
    } else if system_content.contains("summarizer") {
        let count = parse_talent_count_from_user_content(user_content);
        if count == 0 {
            eprintln!("mock_llm: summarizer called but no talents parsed from user content");
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
        let summaries: Vec<Value> = (0..count)
            .map(|i| json!({"talent_index": i, "summary": "Great talent."}))
            .collect();
        serde_json::to_string(&json!({"summaries": summaries})).unwrap()
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };

    Ok(Json(json!({
        "choices": [{"message": {"content": response_content}}]
    })))
}

/// Returns the number of talents in the user message content.
/// The format is: "Prompt: ...\n\nTalents: [...]"
fn parse_talent_count_from_user_content(user_content: &str) -> usize {
    let talents_part = user_content.split("Talents: ").nth(1).unwrap_or("[]");
    let talents: Vec<Value> = serde_json::from_str(talents_part).unwrap_or_default();
    talents.len()
}
