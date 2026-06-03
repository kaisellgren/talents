pub fn normalize_skill(skill: String) -> String {
    if skill.ends_with("designer") {
        return skill.replace("designer", "design").into();
    }
    if skill == "nodejs" {
        return "node.js".into();
    }
    if skill.starts_with("google cloud") {
        return "gcp".into();
    }
    if skill.starts_with("cloud") {
        return "aws".into();
    }
    if skill == "backend engineering" {
        return "backend".into();
    }
    if skill == "frontend engineering" {
        return "frontend".into();
    }
    if skill == "product owner" {
        return "product management".into();
    }
    skill
}