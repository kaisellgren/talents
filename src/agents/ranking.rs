use anyhow::Result;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::talent::Talent;

#[derive(Debug, Clone, Deserialize)]
pub struct RankedTalent {
    pub talent_id: Uuid,
    pub score: f64,
    pub reasoning: String,
}

/// Asks the LLM to rank talents by relevance to the original prompt.
/// Returns talents sorted descending by score.
pub async fn run(talents: &[Talent], prompt: &str) -> Result<Vec<RankedTalent>> {
    if talents.is_empty() {
        return Ok(vec![]);
    }

    let system_prompt = "You are a talent ranking assistant. \
        Given a list of talents and a search prompt, rank the talents by relevance. \
        Consider skills match, location preference, and cost preference as expressed in the prompt. \
        Output JSON: {\"rankings\": [{\"talent_id\": \"<uuid>\", \"score\": <0.0-1.0>, \"reasoning\": \"<brief reason>\"}]}";

    let talents_json = serde_json::to_string(talents)?;
    let user_content = format!("Prompt: {}\n\nTalents: {}", prompt, talents_json);

    let content = crate::llm::chat_completion(system_prompt, &user_content).await?;

    #[derive(Deserialize)]
    struct RankingResponse {
        rankings: Vec<RankedTalent>,
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
