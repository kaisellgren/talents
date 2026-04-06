use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::sync::LazyLock;

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .expect("failed to build HTTP client")
});

/// Sends a chat completion request to the LLM server.
/// Returns the raw content string from the first choice.
pub async fn chat_completion(system_prompt: &str, user_content: &str) -> Result<String> {
    let llm_url =
        std::env::var("LLM_URL").unwrap_or_else(|_| "http://localhost:9000".to_string());
    let model =
        std::env::var("LLM_MODEL").unwrap_or_else(|_| "Qwen/Qwen2.5-3B-Instruct".to_string());

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

    let response = HTTP_CLIENT
        .post(format!("{}/v1/chat/completions", llm_url))
        .json(&body)
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
        .context("Failed to parse SGLang response")?;
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
