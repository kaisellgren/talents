use chrono::Utc;
use sqlx::PgPool;
use std::collections::HashSet;
use talents::db::talent::{Talent, create_talent};
use uuid::Uuid;

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Seeded LCG for deterministic-ish randomness without adding a rand dependency.
struct Rng(u64);

impl Rng {
    fn new() -> Self {
        // Seed from current time so each run differs.
        Self(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64)
    }

    fn next(&mut self) -> u64 {
        // Xorshift64
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn usize_in(&mut self, lo: usize, hi: usize) -> usize {
        lo + (self.next() as usize % (hi - lo + 1))
    }

    fn i32_in(&mut self, lo: i32, hi: i32) -> i32 {
        lo + (self.next() as i32).unsigned_abs() as i32 % (hi - lo + 1)
    }

    fn bool_chance(&mut self, percent: u64) -> bool {
        self.next() % 100 < percent
    }

    /// Sample `n` distinct items from `pool`.
    fn sample<'a, T>(&mut self, pool: &'a [T], n: usize) -> Vec<&'a T> {
        let n = n.min(pool.len());
        let mut indices: Vec<usize> = (0..pool.len()).collect();
        for i in 0..n {
            let j = i + (self.next() as usize % (pool.len() - i));
            indices.swap(i, j);
        }
        indices[..n].iter().map(|&i| &pool[i]).collect()
    }

    fn pick<'a, T>(&mut self, slice: &'a [T]) -> &'a T {
        &slice[self.next() as usize % slice.len()]
    }
}

// ── Data ─────────────────────────────────────────────────────────────────────

struct RoleConfig {
    title: &'static str,
    skills: &'static [&'static str],
    bios: &'static [&'static str],
    rate_min_range: (i32, i32),
    rate_spread: (i32, i32),
}

static ROLES: &[RoleConfig] = &[
    RoleConfig {
        title: "Backend Engineer",
        skills: &["Rust", "Node.js", "TypeScript", "JavaScript", "Scala", "Go", "Python", "Java", "Kotlin", "PostgreSQL", "Redis", "Docker", "Kubernetes", "gRPC", "REST APIs", "GraphQL", "Kafka", "RabbitMQ", "Terraform"],
        bios: &[
            "Backend engineer with {N} years building high-throughput distributed systems. I focus on reliability, clean APIs, and making infrastructure boring in the best possible way.",
            "I've spent {N} years crafting backend services that scale. My sweet spot is designing systems that are easy to reason about and hard to break.",
            "Passionate about performance and correctness. Over {N} years I've built everything from microservices to monoliths — and learned when each is the right tool.",
            "{N} years of backend experience across fintech and SaaS. I care deeply about API design, database efficiency, and keeping on-call alerts quiet.",
            "Based in {City}, I've been writing server-side code for {N} years. I thrive in environments where uptime and data integrity are non-negotiable.",
        ],
        rate_min_range: (70, 110),
        rate_spread: (20, 40),
    },
    RoleConfig {
        title: "Frontend Engineer",
        skills: &["TypeScript", "JavaScript", "React", "Vue.js", "Next.js", "Svelte", "CSS", "HTML", "Tailwind CSS", "Figma", "Webpack", "Vite", "Jest", "Storybook", "Accessibility"],
        bios: &[
            "Frontend engineer with {N} years turning designs into fast, accessible web interfaces. I obsess over user experience, performance budgets, and clean component APIs.",
            "I've spent {N} years building UI systems that designers love and developers don't fear. Strong believer in semantic HTML and progressive enhancement.",
            "{N} years of experience creating polished web products. I bridge the gap between design and engineering, with a focus on pixel-perfect implementation.",
            "Based in {City}, I specialise in React ecosystems and design systems. {N} years of crafting interfaces that are both beautiful and maintainable.",
            "Frontend developer with {N} years of experience. I care about performance, accessibility, and shipping interfaces that actually delight users.",
        ],
        rate_min_range: (60, 95),
        rate_spread: (20, 35),
    },
    RoleConfig {
        title: "Full Stack Engineer",
        skills: &["TypeScript", "React", "Node.js", "PostgreSQL", "Docker", "REST APIs", "Next.js", "GraphQL", "Git", "Redis", "Python", "CSS", "HTML"],
        bios: &[
            "Full stack engineer with {N} years building end-to-end web products. I'm equally comfortable in a React component tree and a PostgreSQL query plan.",
            "{N} years of full stack experience in startup and scale-up environments. I move fast without breaking things — usually.",
            "Generalist engineer based in {City}. Over {N} years I've shipped features across the entire stack, from database schema to responsive UI.",
            "I've spent {N} years owning features top to bottom. Product-minded engineer who enjoys the full picture: database, API, and interface.",
            "Full stack background spanning {N} years. I prefer small teams, high autonomy, and systems that are simple enough to fit in one person's head.",
        ],
        rate_min_range: (65, 100),
        rate_spread: (20, 35),
    },
    RoleConfig {
        title: "Mobile Developer",
        skills: &["Swift", "Kotlin", "React Native", "Flutter", "iOS", "Android", "Firebase", "REST APIs", "Git", "Accessibility", "App Store Connect", "Jetpack Compose"],
        bios: &[
            "Mobile developer with {N} years building native and cross-platform apps. I care about smooth animations, snappy performance, and app store reviews that start with five stars.",
            "{N} years of iOS and Android experience. I've shipped consumer apps used by millions and enterprise tools used daily by field teams.",
            "Based in {City}, I've been building mobile apps for {N} years. My focus is on native feel, offline reliability, and tight design fidelity.",
            "Cross-platform mobile specialist with {N} years of experience. Flutter is my current primary stack, but I'm fluent in Swift and Kotlin too.",
            "{N} years building mobile products from prototype to production. I bridge the gap between product vision and platform constraints.",
        ],
        rate_min_range: (65, 100),
        rate_spread: (20, 35),
    },
    RoleConfig {
        title: "Data Scientist",
        skills: &["Python", "R", "TensorFlow", "PyTorch", "Pandas", "NumPy", "Scikit-learn", "SQL", "Jupyter", "Data Visualization", "Machine Learning", "Statistics", "Spark", "Tableau"],
        bios: &[
            "Data scientist with {N} years extracting insight from messy, real-world datasets. I bridge statistical rigour and practical business impact.",
            "{N} years of experience in predictive modelling and exploratory analysis. I turn ambiguous business questions into clear, measurable answers.",
            "Based in {City}, I've spent {N} years working with data across e-commerce, healthcare, and logistics. I care about reproducibility and interpretability.",
            "Data scientist with a statistics background and {N} years of industry experience. I believe good visualisation is as important as good modelling.",
            "{N} years in data science, working across the full pipeline from raw data ingestion to stakeholder-ready dashboards.",
        ],
        rate_min_range: (70, 105),
        rate_spread: (20, 40),
    },
    RoleConfig {
        title: "ML Engineer",
        skills: &["Python", "TensorFlow", "PyTorch", "MLflow", "Kubernetes", "Docker", "Spark", "Kafka", "Scala", "SQL", "Airflow", "Feature Stores", "Model Serving"],
        bios: &[
            "ML engineer with {N} years building and shipping machine learning systems. I focus on the gap between research prototypes and production models.",
            "{N} years operationalising machine learning — training pipelines, serving infrastructure, monitoring drift. I make models useful, not just accurate.",
            "Based in {City}, I've spent {N} years working on ML platforms at scale. Strong background in both the modelling side and the infrastructure beneath it.",
            "ML engineer with {N} years of experience. My work spans feature engineering, model deployment, and the boring-but-critical tooling that keeps models alive.",
            "{N} years in ML engineering, with a focus on real-time inference and retraining pipelines. I care about latency, reliability, and explainability.",
        ],
        rate_min_range: (80, 120),
        rate_spread: (20, 45),
    },
    RoleConfig {
        title: "DevOps Engineer",
        skills: &["Kubernetes", "Docker", "Terraform", "Ansible", "Jenkins", "GitHub Actions", "Linux", "AWS", "GCP", "Azure", "Prometheus", "Grafana", "Nginx", "Bash", "CI/CD"],
        bios: &[
            "DevOps engineer with {N} years building the infrastructure that keeps software running. I automate everything I touch and monitor everything I automate.",
            "{N} years of cloud and infrastructure experience. I've taken teams from manual deployments to fully automated pipelines — and I enjoy every step of the journey.",
            "Based in {City}, I've been doing DevOps for {N} years. My focus is on reliability, deployment velocity, and keeping the on-call rotation manageable.",
            "Infrastructure engineer with {N} years of experience. I believe infrastructure should be code, deployments should be boring, and alerts should be rare.",
            "{N} years in DevOps across startups and enterprise. Strong opinions on GitOps, observability, and the virtues of a well-tuned Kubernetes cluster.",
        ],
        rate_min_range: (70, 110),
        rate_spread: (20, 40),
    },
    RoleConfig {
        title: "System Architect",
        skills: &["Distributed Systems", "Microservices", "Event-Driven Architecture", "Kafka", "gRPC", "PostgreSQL", "Redis", "Kubernetes", "Cloud Architecture", "System Design", "API Design", "Domain-Driven Design"],
        bios: &[
            "System architect with {N} years designing large-scale distributed systems. I help engineering teams move fast without accumulating architectural debt.",
            "{N} years of experience shaping technical strategy and system design. I thrive at the intersection of business requirements and engineering constraints.",
            "Based in {City}, I've spent {N} years as a hands-on architect. I write ADRs, draw sequence diagrams, and still submit pull requests.",
            "Architect with {N} years of experience across cloud-native and on-premise systems. I specialise in taming complexity through clear boundaries and explicit contracts.",
            "{N} years designing systems that scale. I've made enough mistakes to know which trade-offs matter and which are just flavour of the month.",
        ],
        rate_min_range: (90, 130),
        rate_spread: (25, 50),
    },
    RoleConfig {
        title: "Security Engineer",
        skills: &["Penetration Testing", "OWASP", "Cryptography", "Network Security", "Linux", "Python", "Security Audits", "SIEM", "Zero Trust", "Identity Management", "Threat Modelling", "Bash"],
        bios: &[
            "Security engineer with {N} years finding problems before attackers do. I work across red and blue team disciplines, with a focus on practical, risk-based security.",
            "{N} years in application and infrastructure security. I run threat modelling sessions, review architecture, and occasionally break things for a living.",
            "Based in {City}, I've spent {N} years helping engineering teams build secure systems — not just compliant ones.",
            "Security specialist with {N} years of experience. I believe security is a design constraint, not an afterthought, and I advocate for that at every stage of development.",
            "{N} years in cybersecurity across SaaS and financial services. My approach is pragmatic: fix what matters most, communicate risk clearly, and never stop learning.",
        ],
        rate_min_range: (80, 120),
        rate_spread: (20, 45),
    },
    RoleConfig {
        title: "QA Engineer",
        skills: &["Selenium", "Cypress", "Playwright", "Jest", "Pytest", "Test Automation", "Performance Testing", "API Testing", "CI/CD", "SQL", "Postman", "Load Testing", "BDD"],
        bios: &[
            "QA engineer with {N} years building test automation frameworks and catching the bugs that matter. I believe quality is a team responsibility, not a gatekeeper function.",
            "{N} years in quality assurance. I've shifted testing left in organisations that thought QA meant clicking buttons at the end of a sprint.",
            "Based in {City}, I've spent {N} years designing test strategies and automation suites. I care about fast feedback, meaningful coverage, and developers who trust their pipelines.",
            "QA engineer with {N} years of experience in both manual exploratory testing and automated suites. I find the edge cases that specs forget to mention.",
            "{N} years ensuring software ships with confidence. Strong background in performance testing and setting up test infrastructure from scratch.",
        ],
        rate_min_range: (55, 85),
        rate_spread: (15, 30),
    },
    RoleConfig {
        title: "UX Designer",
        skills: &["Figma", "User Research", "Wireframing", "Prototyping", "Usability Testing", "Design Systems", "Accessibility", "Information Architecture", "Sketch", "Journey Mapping", "Card Sorting"],
        bios: &[
            "UX designer with {N} years championing the user in product decisions. I run research, build prototypes, and push back on assumptions with data.",
            "{N} years designing digital products that people actually want to use. My process starts with understanding real problems before drawing a single wireframe.",
            "Based in {City}, I've spent {N} years as a UX practitioner in both agencies and in-house product teams. I care about evidence-based design.",
            "UX designer with {N} years of experience. I specialise in complex workflows — the kind where getting it wrong has real consequences for real people.",
            "{N} years working at the intersection of user research and interaction design. I bring structure to ambiguity and clarity to complex problems.",
        ],
        rate_min_range: (60, 90),
        rate_spread: (20, 35),
    },
    RoleConfig {
        title: "Graphic Designer",
        skills: &["Figma", "Adobe Illustrator", "Adobe Photoshop", "InDesign", "Branding", "Typography", "Motion Design", "Sketch", "Visual Design", "Brand Identity", "Print Design"],
        bios: &[
            "Graphic designer with {N} years creating visual identities, marketing materials, and digital assets. I believe good design communicates before a word is read.",
            "{N} years of graphic design experience spanning brand identity, editorial, and digital. I work best when creative freedom meets clear strategic goals.",
            "Based in {City}, I've built a career over {N} years working with brands from challenger startups to established European companies.",
            "Visual designer with {N} years of experience. Strong typographic sensibility and a passion for brand systems that scale.",
            "{N} years in graphic design. I've art-directed campaigns, designed packaging, and built visual identities used across print and digital.",
        ],
        rate_min_range: (45, 80),
        rate_spread: (15, 30),
    },
    RoleConfig {
        title: "UI Designer",
        skills: &["Figma", "CSS", "HTML", "Design Systems", "Prototyping", "Sketch", "Adobe XD", "Accessibility", "Motion Design", "Tailwind CSS", "Component Libraries", "Dark Mode"],
        bios: &[
            "UI designer with {N} years crafting interfaces that are both visually precise and technically implementable. I speak fluent designer and fluent developer.",
            "{N} years designing UI for web and mobile. I care about consistency, accessibility, and the small details that make an interface feel considered.",
            "Based in {City}, I've spent {N} years building design systems and component libraries used by cross-functional product teams.",
            "UI designer with {N} years of experience. My work focuses on interaction patterns, visual hierarchy, and bridging the gap between design and code.",
            "{N} years creating UI across SaaS dashboards, e-commerce, and consumer apps. I have a systematic approach to design and strong attention to detail.",
        ],
        rate_min_range: (55, 85),
        rate_spread: (15, 30),
    },
    RoleConfig {
        title: "Product Manager",
        skills: &["Agile", "Scrum", "Roadmapping", "User Research", "Jira", "Confluence", "A/B Testing", "Data Analysis", "Stakeholder Management", "OKRs", "Competitive Analysis", "Product Strategy"],
        bios: &[
            "Product manager with {N} years turning user problems into shipped product. I write crisp specs, run effective discovery, and keep engineering teams unblocked.",
            "{N} years in product management across B2B SaaS and consumer products. I use data to inform decisions and intuition to know which data to look at.",
            "Based in {City}, I've led product teams for {N} years. My approach: understand the problem deeply, set clear outcomes, and trust the team to find the solution.",
            "PM with {N} years of experience. I specialise in 0-to-1 product development and bringing order to roadmaps that have grown without a strategy.",
            "{N} years in product. Strong believer in talking to customers, running structured experiments, and shipping small things often.",
        ],
        rate_min_range: (70, 105),
        rate_spread: (20, 40),
    },
    RoleConfig {
        title: "Product Owner",
        skills: &["Scrum", "Agile", "Backlog Management", "User Stories", "Jira", "Stakeholder Management", "Product Strategy", "Sprint Planning", "Acceptance Criteria", "Confluence"],
        bios: &[
            "Product owner with {N} years keeping backlogs healthy and development teams focused. I translate business goals into user stories that developers can act on immediately.",
            "{N} years as a PO in agile environments. I run tight sprints, write clear acceptance criteria, and keep stakeholder expectations realistic.",
            "Based in {City}, I've been a product owner for {N} years. My job is to make sure the team always knows what to build next and why.",
            "Product owner with {N} years of experience in financial services and e-commerce. I balance competing priorities without losing sight of the user.",
            "{N} years working with Scrum teams as a product owner. Strong skills in stakeholder alignment and turning vague requirements into actionable stories.",
        ],
        rate_min_range: (60, 90),
        rate_spread: (20, 35),
    },
    RoleConfig {
        title: "Project Manager",
        skills: &["Agile", "Scrum", "PMP", "Risk Management", "Budgeting", "MS Project", "Jira", "Confluence", "Stakeholder Management", "Waterfall", "Resource Planning", "Change Management"],
        bios: &[
            "Project manager with {N} years delivering complex projects on time and within budget. I'm the person who keeps chaos organised and stakeholders informed.",
            "{N} years managing software and technology projects. I adapt my approach — agile, waterfall, or hybrid — to what the project and team actually need.",
            "Based in {City}, I've managed projects ranging from small feature teams to multi-vendor programmes spanning {N} years of experience.",
            "PM with {N} years of experience in IT and digital transformation. I'm direct about risks, disciplined about scope, and relentless about delivery.",
            "{N} years in project management. I've rescued projects in trouble and set up new ones for success. Strong communicator at every level of an organisation.",
        ],
        rate_min_range: (60, 90),
        rate_spread: (15, 30),
    },
    RoleConfig {
        title: "SEO Specialist",
        skills: &["SEO", "Google Analytics", "Ahrefs", "Semrush", "Content Strategy", "Technical SEO", "Link Building", "HTML", "WordPress", "Data Analysis", "Google Search Console", "Core Web Vitals"],
        bios: &[
            "SEO specialist with {N} years growing organic traffic for e-commerce, media, and SaaS companies. I work equally on technical foundations and content strategy.",
            "{N} years in SEO across in-house and agency roles. I combine data analysis with an understanding of how search engines actually work.",
            "Based in {City}, I've spent {N} years helping businesses earn visibility in search. My approach is thorough, honest, and grounded in measurable results.",
            "SEO consultant with {N} years of experience. I've led site migrations, built content strategies, and fixed technical issues that were quietly killing rankings.",
            "{N} years in digital marketing with a focus on organic search. I care about sustainable traffic growth built on genuine value, not tricks.",
        ],
        rate_min_range: (50, 80),
        rate_spread: (15, 30),
    },
    RoleConfig {
        title: "System Designer",
        skills: &["System Design", "Distributed Systems", "Cloud Architecture", "Microservices", "UML", "Architecture Patterns", "PostgreSQL", "Kafka", "Documentation", "API Design", "Domain-Driven Design", "Event Sourcing"],
        bios: &[
            "System designer with {N} years creating technical blueprints for complex software systems. I translate business requirements into architectures that are feasible, scalable, and maintainable.",
            "{N} years designing systems at the intersection of product and engineering. I produce documentation that teams actually refer back to.",
            "Based in {City}, I've spent {N} years working on system design across cloud-native platforms and legacy modernisation projects.",
            "Technical designer with {N} years of experience. I specialise in the messy middle ground between high-level architecture and implementation detail.",
            "{N} years in system and solution design. I've shaped platforms from greenfield to production, always with an eye on long-term operability.",
        ],
        rate_min_range: (80, 115),
        rate_spread: (20, 40),
    },
];

static LOCATIONS: &[(&str, &str)] = &[
    ("Helsinki", "Finland"),
    ("Tampere", "Finland"),
    ("Espoo", "Finland"),
    ("Stockholm", "Sweden"),
    ("Gothenburg", "Sweden"),
    ("Malmö", "Sweden"),
    ("Oslo", "Norway"),
    ("Bergen", "Norway"),
    ("Copenhagen", "Denmark"),
    ("Aarhus", "Denmark"),
    ("Amsterdam", "Netherlands"),
    ("Rotterdam", "Netherlands"),
    ("Berlin", "Germany"),
    ("Munich", "Germany"),
    ("Hamburg", "Germany"),
    ("Frankfurt", "Germany"),
    ("Zurich", "Switzerland"),
    ("Geneva", "Switzerland"),
    ("Vienna", "Austria"),
    ("Paris", "France"),
    ("Lyon", "France"),
    ("London", "United Kingdom"),
    ("Manchester", "United Kingdom"),
    ("Edinburgh", "United Kingdom"),
    ("Dublin", "Ireland"),
    ("Barcelona", "Spain"),
    ("Madrid", "Spain"),
    ("Lisbon", "Portugal"),
    ("Milan", "Italy"),
    ("Rome", "Italy"),
    ("Warsaw", "Poland"),
    ("Kraków", "Poland"),
    ("Prague", "Czech Republic"),
    ("Budapest", "Hungary"),
    ("Brussels", "Belgium"),
    ("Tallinn", "Estonia"),
    ("Riga", "Latvia"),
    ("Vilnius", "Lithuania"),
    ("Bucharest", "Romania"),
    ("Sofia", "Bulgaria"),
];

static FIRST_NAMES: &[&str] = &[
    // Finnish / Scandinavian
    "Mikko", "Janne", "Sanna", "Aino", "Eetu", "Hanna", "Ville", "Laura", "Petri", "Tiina",
    "Erik", "Astrid", "Lars", "Ingrid", "Bjorn", "Sigrid", "Nils", "Frida", "Olaf", "Maja",
    // German / Austrian / Swiss
    "Felix", "Hannah", "Lukas", "Sophie", "Jonas", "Emma", "Tobias", "Lena", "Markus", "Julia",
    "Sebastian", "Anna", "Fabian", "Marie", "Maximilian",
    // British / Irish
    "James", "Emily", "Oliver", "Charlotte", "Harry", "Amelia", "George", "Isla", "Liam", "Evie",
    // French
    "Pierre", "Camille", "Antoine", "Léa", "Hugo", "Manon",
    // Spanish / Portuguese
    "Carlos", "María", "Javier", "Isabel", "Miguel", "Sofía", "Rafael", "Lucía",
    // Italian
    "Marco", "Giulia", "Luca", "Valentina", "Matteo", "Chiara",
    // Eastern European
    "Piotr", "Katarzyna", "Tomáš", "Petra", "Andrei", "Elena", "Matej", "Zuzana",
];

static LAST_NAMES: &[&str] = &[
    // Finnish
    "Virtanen", "Mäkinen", "Nieminen", "Mäkinen", "Hämäläinen", "Leinonen", "Korhonen",
    // Scandinavian
    "Eriksson", "Lindqvist", "Johansson", "Andersen", "Nielsen", "Hansen", "Berg", "Holm",
    // German
    "Müller", "Schmidt", "Weber", "Fischer", "Meyer", "Wagner", "Hoffmann", "Becker",
    // British
    "Smith", "Jones", "Williams", "Taylor", "Brown", "Davies", "Evans", "Wilson",
    // French
    "Martin", "Bernard", "Dubois", "Laurent", "Lefebvre", "Moreau",
    // Spanish
    "García", "Rodríguez", "López", "Fernández", "Martínez", "Sánchez",
    // Italian
    "Rossi", "Russo", "Ferrari", "Esposito", "Bianchi", "Romano",
    // Eastern European
    "Kowalski", "Novák", "Ionescu", "Popescu", "Kovács", "Nagy",
];

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    let mut rng = Rng::new();
    let mut name_set: HashSet<String> = HashSet::new();
    let mut inserted = 0usize;

    println!("Seeding 1000 talents…");

    while inserted < 1000 {
        let role = rng.pick(ROLES);
        let (city, country) = *rng.pick(LOCATIONS);
        let first = rng.pick(FIRST_NAMES);
        let last = rng.pick(LAST_NAMES);
        let name = format!("{first} {last}");

        // Keep names unique.
        if name_set.contains(&name) {
            continue;
        }
        name_set.insert(name.clone());

        let skill_count = rng.usize_in(3, 6);
        let skills: Vec<String> = rng
            .sample(role.skills, skill_count)
            .into_iter()
            .map(|s| s.to_string().to_ascii_lowercase())
            .collect();

        let years: i32 = rng.i32_in(3, 16);
        let bio_template = rng.pick(role.bios);
        let biography = bio_template
            .replace("{N}", &years.to_string())
            .replace("{City}", city);

        let rate_min = rng.i32_in(role.rate_min_range.0, role.rate_min_range.1);
        let rate_spread = rng.i32_in(role.rate_spread.0, role.rate_spread.1);
        let rate_max = rate_min + rate_spread;

        let available = rng.bool_chance(65);

        let talent = Talent {
            id: Uuid::new_v4(),
            name,
            skills,
            location_city: city.to_string(),
            location_country: country.to_string(),
            role: Some(role.title.to_string()),
            available,
            hourly_rate_min: Some(rate_min),
            hourly_rate_max: Some(rate_max),
            biography: Some(biography),
            created_at: Utc::now(),
        };

        create_talent(&pool, talent).await?;
        inserted += 1;

        if inserted % 100 == 0 {
            println!("  {inserted}/1000");
        }
    }

    println!("Done. {inserted} talents inserted.");
    Ok(())
}
