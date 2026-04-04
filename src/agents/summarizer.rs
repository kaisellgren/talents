use anyhow::Result;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::candidate::Candidate;

#[derive(Debug, Clone, Deserialize)]
pub struct SummarizedCandidate {
    pub candidate_id: Uuid,
    pub summary: String,
}

/// Asks the LLM to generate a short per-candidate summary explaining
/// why each candidate suits the original prompt.
pub async fn run(candidates: &[Candidate], prompt: &str) -> Result<Vec<SummarizedCandidate>> {
    let system_prompt = "You are a talent summarizer. \
        For each candidate, write a 2-3 sentence summary explaining why they are well-suited \
        for the given search prompt. Be specific about their skills, location, and rate. \
        Output JSON: {\"summaries\": [{\"candidate_id\": \"<uuid>\", \"summary\": \"<text>\"}]}";

    let candidates_json = serde_json::to_string(candidates)?;
    let user_content = format!("Prompt: {}\n\nCandidates: {}", prompt, candidates_json);

    let content = crate::sglang::chat_completion(system_prompt, &user_content).await?;

    #[derive(Deserialize)]
    struct SummaryResponse {
        summaries: Vec<SummarizedCandidate>,
    }

    let response: SummaryResponse = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Summarizer agent returned invalid JSON: {}: {}", e, content))?;

    Ok(response.summaries)
}