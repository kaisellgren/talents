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
                && let Some(talent_max) = c.hourly_rate_max
                && talent_max > max_rate
            {
                return false;
                // Talents with no hourly_rate_max set are allowed through —
                // rate is unknown, so we do not assume a violation.
            }
            let skills_lower: Vec<String> =
                c.skills.iter().map(|s| s.to_ascii_lowercase()).collect();
            triage
                .required_skills
                .iter()
                .all(|req| skills_lower.contains(&req.to_ascii_lowercase()))
        })
        .collect()
}
