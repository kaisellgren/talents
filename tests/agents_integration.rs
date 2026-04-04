use talents::agents::constraint;
use talents::agents::triage::TriageOutput;
use talents::db::candidate::Candidate;
use chrono::Utc;
use uuid::Uuid;

fn make_candidate(
    skills: Vec<String>,
    available: bool,
    hourly_rate_max: Option<i32>,
) -> Candidate {
    Candidate {
        id: Uuid::new_v4(),
        name: "Test".to_string(),
        skills,
        location_city: "Helsinki".to_string(),
        location_country: "Finland".to_string(),
        role: None,
        available,
        hourly_rate_min: None,
        hourly_rate_max,
        biography: None,
        created_at: Utc::now(),
    }
}

fn make_triage(required: Vec<&str>, max_rate: Option<i32>) -> TriageOutput {
    TriageOutput {
        required_skills: required.into_iter().map(String::from).collect(),
        preferred_skills: vec![],
        location_city: None,
        location_country: None,
        max_hourly_rate: max_rate,
    }
}

#[test]
fn keeps_available_candidate_matching_skills_and_rate() {
    let candidates = vec![make_candidate(vec!["rust".into()], true, Some(100))];
    let triage = make_triage(vec!["rust"], Some(100));
    let result = constraint::run(candidates, &triage);
    assert_eq!(result.len(), 1);
}

#[test]
fn removes_unavailable_candidate() {
    let candidates = vec![make_candidate(vec!["rust".into()], false, Some(100))];
    let triage = make_triage(vec!["rust"], None);
    let result = constraint::run(candidates, &triage);
    assert!(result.is_empty());
}

#[test]
fn removes_candidate_exceeding_max_rate() {
    let candidates = vec![make_candidate(vec!["rust".into()], true, Some(200))];
    let triage = make_triage(vec!["rust"], Some(100));
    let result = constraint::run(candidates, &triage);
    assert!(result.is_empty());
}

#[test]
fn removes_candidate_missing_required_skill() {
    let candidates = vec![make_candidate(vec!["python".into()], true, Some(80))];
    let triage = make_triage(vec!["rust"], None);
    let result = constraint::run(candidates, &triage);
    assert!(result.is_empty());
}

#[test]
fn keeps_candidate_when_no_rate_limit_specified() {
    let candidates = vec![make_candidate(vec!["rust".into()], true, None)];
    let triage = make_triage(vec!["rust"], None);
    let result = constraint::run(candidates, &triage);
    assert_eq!(result.len(), 1);
}
