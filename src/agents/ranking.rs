use anyhow::Result;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::candidate::Candidate;

#[derive(Debug, Clone, Deserialize)]
pub struct RankedCandidate {
    pub candidate_id: Uuid,
    pub score: f64,
    pub reasoning: String,
}

/// Asks the LLM to rank candidates by relevance to the original prompt.
/// Returns candidates sorted descending by score.
pub async fn run(candidates: &[Candidate], prompt: &str) -> Result<Vec<RankedCandidate>> {
    if candidates.is_empty() {
        return Ok(vec![]);
    }

    let system_prompt = "You are a talent ranking assistant. \
        Given a list of candidates and a search prompt, rank the candidates by relevance. \
        Consider skills match, location preference, and cost preference as expressed in the prompt. \
        Output JSON: {\"rankings\": [{\"candidate_id\": \"<uuid>\", \"score\": <0.0-1.0>, \"reasoning\": \"<brief reason>\"}]}";

    let candidates_json = serde_json::to_string(candidates)?;
    let user_content = format!("Prompt: {}\n\nCandidates: {}", prompt, candidates_json);

    let content = crate::sglang::chat_completion(system_prompt, &user_content).await?;

    #[derive(Deserialize)]
    struct RankingResponse {
        rankings: Vec<RankedCandidate>,
    }

    let response: RankingResponse = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Ranking agent returned invalid JSON: {}: {}", e, content))?;

    let mut rankings = response.rankings;
    rankings
        .iter_mut()
        .for_each(|r| r.score = r.score.clamp(0.0, 1.0));
    rankings.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    Ok(rankings)
}
