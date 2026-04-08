use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct SummarizedTalent {
    pub talent_index: usize,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SummarizerTalentInput {
    pub talent_index: usize,
    pub name: String,
    pub skills: Vec<String>,
    pub location: String,
    pub role: Option<String>,
}

/// Asks the LLM to generate a short per-talent summary explaining
/// why each talent suits the original prompt.
pub async fn run(
    talents: &[SummarizerTalentInput],
    prompt: &str,
) -> Result<Vec<SummarizedTalent>> {
    if talents.is_empty() {
        return Ok(vec![]);
    }

    let system_prompt = "You are a talent summarizer. Write 2-3 sentence summaries focused on skills and location. Do not mention rate, price, cost, or fees. Return JSON only: {\"summaries\":[{\"talent_index\":0,\"summary\":\"...\"}]}";

    let talents_json = serde_json::to_string(talents)?;
    let user_content = format!("Prompt: {}\n\nTalents: {}", prompt, talents_json);

    let content = crate::llm::chat_completion(system_prompt, &user_content).await?;

    #[derive(Deserialize)]
    struct SummaryResponse {
        summaries: Vec<SummarizedTalent>,
    }

    let response: SummaryResponse = serde_json::from_str(&content).map_err(|e| {
        anyhow::anyhow!("Summarizer agent returned invalid JSON: {}: {}", e, content)
    })?;

    Ok(response.summaries)
}
