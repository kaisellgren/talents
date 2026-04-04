use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::sync::LazyLock;

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .expect("failed to build HTTP client")
});

/// Sends a chat completion request to the SGLang server.
/// Returns the raw content string from the first choice.
pub async fn chat_completion(system_prompt: &str, user_content: &str) -> Result<String> {
    let sglang_url = std::env::var("SGLANG_URL")
        .unwrap_or_else(|_| "http://localhost:9000".to_string());

    let body = json!({
        "model": "default",
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_content}
        ],
        "response_format": {"type": "json_object"}
    });

    let response = HTTP_CLIENT
        .post(format!("{}/v1/chat/completions", sglang_url))
        .json(&body)
        .send()
        .await
        .context("Failed to reach SGLang server")?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("SGLang returned {}: {}", status, text);
    }

    let response_body: Value = response.json().await.context("Failed to parse SGLang response")?;
    let content = response_body["choices"][0]["message"]["content"]
        .as_str()
        .context("Missing content in SGLang response")?
        .to_string();

    Ok(content)
}
