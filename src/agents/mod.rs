pub mod constraint;
pub mod ranking;
pub mod research;
pub mod summarizer;
pub mod triage;

use anyhow::Result;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct AgentCandidate {
    pub id: Uuid,
    pub name: String,
    pub score: f64,
    pub reasoning: String,
    pub summary: String,
}

#[derive(Debug, Serialize)]
pub struct AgentResponse {
    pub candidates: Vec<AgentCandidate>,
    pub iterations: u32,
}

/// Runs the full agentic loop: triage → research → constraint → ranking → summarizer.
/// Retries up to 5 times with broadened keywords if constraint produces no candidates.
pub async fn run_agent_loop(pool: &PgPool, prompt: &str) -> Result<AgentResponse> {
    let max_iterations = 5u32;
    let mut previous_required_skills: Option<Vec<String>> = None;

    for iteration in 1..=max_iterations {
        let triage = triage::run(prompt, previous_required_skills.as_deref()).await?;
        let candidates = research::run(pool, &triage).await?;
        let filtered = constraint::run(candidates, &triage);

        if filtered.is_empty() {
            if iteration == max_iterations {
                return Ok(AgentResponse {
                    candidates: vec![],
                    iterations: iteration,
                });
            }
            previous_required_skills = Some(triage.required_skills.clone());
            continue;
        }

        let rankings = ranking::run(&filtered, prompt).await?;
        let summaries = summarizer::run(&filtered, prompt).await?;

        let candidates = rankings
            .into_iter()
            .filter_map(|r| {
                let candidate = filtered.iter().find(|c| c.id == r.candidate_id);
                if candidate.is_none() {
                    eprintln!("ranking agent returned unknown candidate_id: {}", r.candidate_id);
                }
                let candidate = candidate?;
                let summary = summaries
                    .iter()
                    .find(|s| s.candidate_id == r.candidate_id)
                    .map(|s| s.summary.clone())
                    .unwrap_or_default();
                Some(AgentCandidate {
                    id: candidate.id,
                    name: candidate.name.clone(),
                    score: r.score,
                    reasoning: r.reasoning,
                    summary,
                })
            })
            .collect();

        return Ok(AgentResponse {
            candidates,
            iterations: iteration,
        });
    }

    Ok(AgentResponse {
        candidates: vec![],
        iterations: max_iterations,
    })
}
