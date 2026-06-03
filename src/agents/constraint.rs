use crate::agents::skill_normalization::normalize_skill;
use crate::agents::triage::TriageOutput;
use crate::db::talent::Talent;

/// Filters talents using pure Rust logic — no LLM call.
/// Removes talents that are unavailable, exceed the max hourly rate,
/// or are missing any required skill.
pub fn run(talents: Vec<Talent>, triage: &TriageOutput) -> Vec<Talent> {
    talents
        .into_iter()
        .filter(|c| {
            if !c.available {
                return false;
            }
            if let Some(max_rate) = triage.max_hourly_rate
                && c.hourly_rate > max_rate
            {
                return false;
            }
            let skills_lower: Vec<String> =
                c.skills.iter().map(|s| s.to_ascii_lowercase()).collect();
            triage
                .required_skills
                .iter()
                .map(|s| normalize_skill(s.trim().to_ascii_lowercase()))
                .all(|req| skills_lower.contains(&req.to_ascii_lowercase()))
        })
        .collect()
}
