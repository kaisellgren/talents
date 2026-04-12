use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::sync::LazyLock;

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .expect("failed to build HTTP client")
});

/// Fetches a GCP access token from the instance metadata server.
/// Only called when USE_GCP_AUTH=true.
async fn fetch_gcp_access_token() -> Result<String> {
    let resp: Value = HTTP_CLIENT
        .get("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token")
        .header("Metadata-Flavor", "Google")
        .send()
        .await
        .context("Failed to reach GCP metadata server")?
        .json()
        .await
        .context("Failed to parse GCP metadata token response")?;
    resp["access_token"]
        .as_str()
        .map(|s| s.to_string())
        .context("Missing access_token in GCP metadata response")
}

/// Sends a chat completion request to the LLM server.
/// Returns the raw content string from the first choice.
pub async fn chat_completion(system_prompt: &str, user_content: &str) -> Result<String> {
    let llm_url =
        std::env::var("LLM_URL").unwrap_or_else(|_| "http://localhost:9000".to_string());
    let model =
        std::env::var("LLM_MODEL").unwrap_or_else(|_| "Qwen/Qwen2.5-3B-Instruct".to_string());
    let use_gcp_auth = std::env::var("USE_GCP_AUTH").unwrap_or_default() == "true";

    let system_prompt_with_json = format!(
        "{}\n\nYou MUST respond with valid JSON only. Do not include any explanation, markdown, or text outside the JSON object.",
        system_prompt
    );

    let body = json!({
        "model": model,
        "messages": [
            {"role": "system", "content": system_prompt_with_json},
            {"role": "user", "content": user_content}
        ]
    });

    let mut request = HTTP_CLIENT
        .post(format!("{}/chat/completions", llm_url))
        .json(&body);

    if use_gcp_auth {
        let token = fetch_gcp_access_token().await?;
        request = request.bearer_auth(token);
    }

    // LLM_URL is the OpenAI-compatible base (e.g. "http://localhost:1234/v1" or
    // the Vertex AI openapi endpoint). Always append just "/chat/completions".
    let response = request
        .send()
        .await
        .context("Failed to reach LLM server")?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        anyhow::bail!("LLM server returned {}: {}", status, text);
    }

    let response_body: Value = response
        .json()
        .await
        .context("Failed to parse LLM response")?;
    let content = response_body["choices"][0]["message"]["content"]
        .as_str()
        .context("Missing content in LLM response")?
        .to_string();

    tracing::debug!("LLM raw response: {}", content);

    // Strip any leading control tokens / metadata (e.g. LM Studio's <|channel|>...<|message|>)
    // by finding the first JSON start character.
    let json_start = content
        .find(|c| c == '{' || c == '[')
        .context("No JSON object found in LLM response")?;
    let content = content[json_start..].to_string();

    Ok(content)
}
