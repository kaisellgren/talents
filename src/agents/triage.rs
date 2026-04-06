use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriageOutput {
    pub required_skills: Vec<String>,
    #[serde(default)]
    pub preferred_skills: Vec<String>,
    pub location_city: Option<String>,
    pub location_country: Option<String>,
    pub max_hourly_rate: Option<i32>,
}

/// Calls the triage agent to extract structured search criteria from a prompt.
/// Pass `previous_required_skills` on retry to instruct the LLM to broaden the search.
pub async fn run(
    prompt: &str,
    previous_required_skills: Option<&[String]>,
) -> Result<TriageOutput> {
    let system_prompt = if let Some(prev) = previous_required_skills {
        format!(
            "You are a talent search triage assistant. Extract search criteria from the prompt as JSON.\n\
            A previous search with required_skills {:?} returned no results. \
            Produce FEWER required_skills (broaden the search) while keeping the most important ones.\n\
            Output only JSON with keys: required_skills (array), preferred_skills (array), \
            location_city (string or null), location_country (string or null), max_hourly_rate (number or null).",
            prev
        )
    } else {
        "You are a talent search triage assistant. Extract search criteria from the prompt as JSON.\n\
        Output only JSON with keys: required_skills (array of lowercase skill strings), \
        preferred_skills (array of lowercase skill strings, used for ranking only), \
        location_city (string or null), location_country (string or null), \
        max_hourly_rate (number or null).".to_string()
    };

    let content = crate::sglang::chat_completion(&system_prompt, prompt).await?;
    let output: TriageOutput = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Triage agent returned invalid JSON: {}: {}", e, content))?;
    Ok(output)
}
