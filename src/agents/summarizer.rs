use anyhow::Result;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::talent::Talent;

#[derive(Debug, Clone, Deserialize)]
pub struct SummarizedTalent {
    pub talent_id: Uuid,
    pub summary: String,
}

/// Asks the LLM to generate a short per-talent summary explaining
/// why each talent suits the original prompt.
pub async fn run(talents: &[Talent], prompt: &str) -> Result<Vec<SummarizedTalent>> {
    if talents.is_empty() {
        return Ok(vec![]);
    }

    let system_prompt = "You are a talent summarizer. \
        For each talent, write a 2-3 sentence summary explaining why they are well-suited \
        for the given search prompt. Be specific about their skills and location. Never mention rate, price, cost or fees. \
        Output must be the following JSON including the talent_id field: {\"summaries\": [{\"talent_id\": \"<uuid>\", \"summary\": \"<text>\"}]}";

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
