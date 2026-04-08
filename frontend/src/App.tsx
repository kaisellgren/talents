import { useState, useRef } from 'react';
import { Routes, Route, NavLink } from 'react-router-dom';
import { LayoutGrid, Search, Sparkles } from 'lucide-react';
import { runAgent } from './api';
import type { AgentResponse, AgentTalent, Talent } from './types';
import { TalentCard } from './components/TalentCard';
import { CatalogPage } from './pages/CatalogPage';
import * as s from './App.css';

const PROMPTS: { title: string; text: string }[] = [
  { title: 'Lead Product Designer',      text: 'I need a lead product designer to own our end-to-end design process and work closely with engineering' },
  { title: 'Rust Systems Engineer',      text: 'Looking for a Rust engineer to build high-performance backend services' },
  { title: 'React Native Developer',     text: 'I am looking for a mobile developer who can build cross-platform apps using React Native' },
  { title: 'DevOps Platform Engineer',   text: 'We need a DevOps engineer experienced with Kubernetes and CI/CD pipelines' },
  { title: 'Senior UX Designer',         text: 'Looking for a senior UX designer who can run user research and design complex workflows' },
  { title: 'Full Stack Go Developer',    text: 'I need a full stack developer with strong Go backend skills and React frontend experience' },
  { title: 'Machine Learning Engineer',  text: 'We are looking for an ML engineer who can take models from prototype to production' },
  { title: 'iOS Mobile Architect',       text: 'Looking for an experienced iOS developer who can architect and lead mobile development' },
  { title: 'Data Visualisation Expert',  text: 'I need someone who can turn complex datasets into clear, interactive dashboards' },
  { title: 'Cybersecurity Specialist',   text: 'Looking for a security engineer to audit our infrastructure and run penetration tests' },
  { title: 'Principal Product Manager',  text: 'We need a principal PM who can define product strategy and align cross-functional teams' },
  { title: 'Graphic Designer',           text: 'Looking for a graphic designer to refresh our brand identity and create marketing materials' },
  { title: 'SEO Specialist',             text: 'I need an SEO specialist who can improve our organic search rankings and fix technical SEO issues' },
  { title: 'System Architect',           text: 'We are looking for a system architect to design our microservices migration strategy' },
  { title: 'QA Automation Engineer',     text: 'Looking for a QA engineer to build end-to-end test automation with Playwright' },
  { title: 'Project Manager',            text: 'I need an experienced project manager to lead a cross-functional delivery team' },
  { title: 'UI Designer',                text: 'Looking for a UI designer who is strong in design systems and can work closely with developers' },
  { title: 'Backend Engineer',           text: 'We need a backend engineer to scale our PostgreSQL-backed API to handle more traffic' },
  { title: 'Product Owner',              text: 'Looking for a product owner experienced in Scrum to manage our development backlog' },
  { title: 'Data Scientist',             text: 'I need a data scientist who can build predictive models and communicate findings to stakeholders' },
];

function SearchPage() {
  const [prompt, setPrompt] = useState('');
  const [loading, setLoading] = useState(false);
  const [agentResult, setAgentResult] = useState<AgentResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const resultsRef = useRef<HTMLDivElement>(null);

  async function handleSearch(e: React.FormEvent) {
    e.preventDefault();
    if (!prompt.trim()) return;
    setLoading(true);
    setError(null);
    setAgentResult(null);
    try {
      const res = await runAgent(prompt.trim());
      setAgentResult(res);
      resultsRef.current?.scrollIntoView({ behavior: 'smooth' });
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }

  type DisplayItem =
    | { talent: Talent; agent: AgentTalent }
    | { talent: Talent; agent?: undefined };

  const displayItems: DisplayItem[] = agentResult
    ? agentResult.talents.map((ac) => ({
        talent: {
          id: ac.id,
          name: ac.name,
          skills: ac.skills,
          location_city: ac.location_city,
          location_country: ac.location_country,
          role: ac.role,
          available: true,
          hourly_rate: ac.hourly_rate,
          biography: ac.biography,
          created_at: '',
        },
        agent: ac,
      }))
    : [];

  return (
    <>
      <section id="search" className={s.heroSection}>
        <svg className={s.logoMark} viewBox="0 0 36 36" fill="none" aria-hidden="true" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <linearGradient id="lt" x1="18" y1="3" x2="18" y2="18" gradientUnits="userSpaceOnUse">
              <stop stopColor="#e8ecff"/>
              <stop offset="1" stopColor="#a3a6ff"/>
            </linearGradient>
            <linearGradient id="ll" x1="3" y1="13" x2="18" y2="33" gradientUnits="userSpaceOnUse">
              <stop stopColor="#9396f5"/>
              <stop offset="1" stopColor="#4e51c4"/>
            </linearGradient>
            <linearGradient id="lr" x1="33" y1="13" x2="18" y2="33" gradientUnits="userSpaceOnUse">
              <stop stopColor="#7b7ee8"/>
              <stop offset="1" stopColor="#3d40b0"/>
            </linearGradient>
          </defs>
          <polygon points="18,3 31,13 18,19 5,13" fill="url(#lt)"/>
          <polygon points="5,13 18,19 18,33 3,22" fill="url(#ll)"/>
          <polygon points="31,13 18,19 18,33 33,22" fill="url(#lr)"/>
          <polygon points="18,3 31,13 18,19 5,13" fill="none" stroke="rgba(255,255,255,0.18)" strokeWidth="0.6"/>
        </svg>
        <p className={s.heroEyebrow}>Elite Talent Discovery</p>
        <h1 className={s.heroTitle}>Talents</h1>
        <p className={s.heroSubtitle}>
          Discover exceptional talent through AI-powered natural language search.
          <br />
          <span className={s.heroNote}>This is a demo</span>
        </p>

        <form onSubmit={handleSearch} className={s.searchForm}>
          <div className={s.searchInputShell}>
            <Search className={s.searchInputIcon} aria-hidden="true" />
            <textarea
              className={s.searchInput}
              value={prompt}
              onChange={(e) => setPrompt(e.target.value)}
              placeholder="Describe the talent you're looking for…"
              disabled={loading}
              rows={2}
              autoFocus
            />
          </div>
          <button className={s.searchButton} type="submit" disabled={loading || !prompt.trim()}>
            <Sparkles className={s.buttonIcon} aria-hidden="true" />
            <span>{loading ? 'Searching…' : 'Search with AI'}</span>
          </button>
        </form>

        <div className={s.promptCloud}>
          <p className={s.promptCloudLabel}>AI Prompt Cloud — Click to explore</p>
          <div className={s.promptChips}>
            {PROMPTS.map((p) => (
              <button
                key={p.title}
                className={s.promptChip}
                type="button"
                onClick={() => setPrompt(p.text)}
              >
                {p.title}
              </button>
            ))}
          </div>
        </div>

        {loading && (
          <p className={s.statusMsg}>
            <span className={s.statusDot} />
            determining required skills…
          </p>
        )}
        {error && <p className={s.errorMsg}>{error}</p>}
      </section>

      <div ref={resultsRef} className={s.catalogSection}>
        {agentResult !== null && displayItems.length == 0 && (
            <p className={s.catalogSection}>No results found. Try another query.</p>
        )}
        {agentResult !== null && displayItems.length > 0 && (
          <>
            <div className={s.catalogHeader}>
              <h2 className={s.catalogTitle}>
                {agentResult.talents.length} result{agentResult.talents.length !== 1 ? 's' : ''}{' '}
                · {agentResult.iterations} iteration{agentResult.iterations !== 1 ? 's' : ''}
              </h2>
            </div>
            <div className={s.grid}>
              {displayItems.map(({ talent, agent }) => (
                <TalentCard key={talent.id} talent={talent} agent={agent} />
              ))}
            </div>
          </>
        )}
      </div>
    </>
  );
}

function App() {
  return (
    <div className={s.root}>
      <div className={s.glowTopRight} />
      <div className={s.glowBottomLeft} />
      <div className={s.glowHeroCenter} />
      <div className={s.actionZoneBlur} />

      <header className={s.header}>
        <nav className={s.nav}>
          <NavLink to="/" end className={({ isActive }) => isActive ? s.navLinkActive : s.navLink}>
            <span className={s.navLinkContent}>
              <Search className={s.navIcon} aria-hidden="true" />
              <span>Search</span>
            </span>
          </NavLink>
          <NavLink to="/catalog" className={({ isActive }) => isActive ? s.navLinkActive : s.navLink}>
            <span className={s.navLinkContent}>
              <LayoutGrid className={s.navIcon} aria-hidden="true" />
              <span>Catalog</span>
            </span>
          </NavLink>
        </nav>
      </header>

      <Routes>
        <Route path="/" element={<SearchPage />} />
        <Route path="/catalog" element={<CatalogPage />} />
      </Routes>
    </div>
  );
}

export default App;
