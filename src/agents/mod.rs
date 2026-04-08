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
pub struct AgentTalent {
    pub id: Uuid,
    pub name: String,
    pub score: f64,
    pub reasoning: String,
    pub summary: String,
    pub skills: Vec<String>,
    pub location_city: String,
    pub location_country: String,
    pub role: Option<String>,
    pub hourly_rate: i32,
    pub biography: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AgentResponse {
    pub talents: Vec<AgentTalent>,
    pub iterations: u32,
}

/// Runs the full agentic loop: triage → research → constraint → ranking → summarizer.
/// Retries up to 5 times with broadened keywords if constraint produces no talents.
pub async fn run_agent_loop(pool: &PgPool, prompt: &str) -> Result<AgentResponse> {
    let max_iterations = 5u32;
    let mut previous_required_skills: Option<Vec<String>> = None;

    for iteration in 1..=max_iterations {
        let triage = triage::run(prompt, previous_required_skills.as_deref()).await?;
        let talents = research::run(pool, &triage).await?;
        let filtered = constraint::run(talents, &triage);

        if filtered.is_empty() {
            if iteration == max_iterations {
                return Ok(AgentResponse {
                    talents: vec![],
                    iterations: iteration,
                });
            }
            previous_required_skills = Some(triage.required_skills.clone());
            continue;
        }

        let rankings = ranking::run(&filtered, prompt).await?;
        let summaries = summarizer::run(&filtered, prompt).await?;

        let talents = rankings
            .into_iter()
            .filter_map(|r| {
                let talent = filtered.get(r.talent_index);
                if talent.is_none() {
                    eprintln!(
                        "ranking agent returned unknown talent index: {}",
                        r.talent_index
                    );
                }
                let talent = talent?;
                let summary_entry = summaries
                    .iter()
                    .find(|s| s.talent_index == r.talent_index);
                if summary_entry.is_none() {
                    eprintln!(
                        "summarizer agent returned no summary for talent index: {}",
                        r.talent_index
                    );
                }
                let summary = summary_entry.map(|s| s.summary.clone()).unwrap_or_default();
                Some(AgentTalent {
                    id: talent.id,
                    name: talent.name.clone(),
                    score: r.score,
                    reasoning: r.reasoning,
                    summary,
                    skills: talent.skills.clone(),
                    location_city: talent.location_city.clone(),
                    location_country: talent.location_country.clone(),
                    role: talent.role.clone(),
                    hourly_rate: talent.hourly_rate,
                    biography: talent.biography.clone(),
                })
            })
            .collect();

        return Ok(AgentResponse {
            talents: talents,
            iterations: iteration,
        });
    }

    Ok(AgentResponse {
        talents: vec![],
        iterations: max_iterations,
    })
}
