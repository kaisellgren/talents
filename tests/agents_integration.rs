use chrono::Utc;
use talents::agents::constraint;
use talents::agents::triage::TriageOutput;
use talents::db::talent::Talent;
use uuid::Uuid;

fn make_talent(skills: Vec<String>, available: bool, hourly_rate: i32) -> Talent {
    Talent {
        id: Uuid::new_v4(),
        name: "Test".to_string(),
        skills,
        location_city: "Helsinki".to_string(),
        location_country: "Finland".to_string(),
        role: None,
        available,
        hourly_rate,
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
fn keeps_available_talent_matching_skills_and_rate() {
    let talents = vec![make_talent(vec!["rust".into()], true, 100)];
    let triage = make_triage(vec!["rust"], Some(100));
    let result = constraint::run(talents, &triage);
    assert_eq!(result.len(), 1);
}

#[test]
fn removes_unavailable_talent() {
    let talents = vec![make_talent(vec!["rust".into()], false, 100)];
    let triage = make_triage(vec!["rust"], None);
    let result = constraint::run(talents, &triage);
    assert!(result.is_empty());
}

#[test]
fn removes_talent_exceeding_max_rate() {
    let talents = vec![make_talent(vec!["rust".into()], true, 200)];
    let triage = make_triage(vec!["rust"], Some(100));
    let result = constraint::run(talents, &triage);
    assert!(result.is_empty());
}

#[test]
fn removes_talent_missing_required_skill() {
    let talents = vec![make_talent(vec!["python".into()], true, 80)];
    let triage = make_triage(vec!["rust"], None);
    let result = constraint::run(talents, &triage);
    assert!(result.is_empty());
}

#[test]
fn keeps_talent_when_no_rate_limit_specified() {
    let talents = vec![make_talent(vec!["rust".into()], true, 150)];
    let triage = make_triage(vec!["rust"], None);
    let result = constraint::run(talents, &triage);
    assert_eq!(result.len(), 1);
}

#[test]
fn keeps_talent_within_max_rate() {
    let talents = vec![make_talent(vec!["rust".into()], true, 100)];
    let triage = make_triage(vec!["rust"], Some(100));
    let result = constraint::run(talents, &triage);
    assert_eq!(result.len(), 1);
}
