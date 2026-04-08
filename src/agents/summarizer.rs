use anyhow::Result;
use serde::Deserialize;

use crate::db::talent::Talent;

#[derive(Debug, Clone, Deserialize)]
pub struct SummarizedTalent {
    pub talent_index: usize,
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
        Output must be the following JSON including the talent_index field: {\"summaries\": [{\"talent_index\": <zero-based integer>, \"summary\": \"<text>\"}]}. \
        Use the zero-based index of the talent in the provided list. Do not invent or modify IDs.";

    let talents_json = serde_json::to_string_pretty(talents)?;
    let user_content = format!(
        "Prompt: {}\n\nTalents are listed in order. Use the zero-based position as talent_index.\n\nTalents: {}",
        prompt, talents_json
    );

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
