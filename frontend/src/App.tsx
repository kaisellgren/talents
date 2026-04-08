import { useState, useRef } from 'react';
import { Routes, Route, NavLink } from 'react-router-dom';
import { LayoutGrid, Search, Sparkles } from 'lucide-react';
import { runAgent } from './api';
import type { AgentResponse, AgentTalent, Talent } from './types';
import { TalentCard } from './components/TalentCard';
import { CatalogPage } from './pages/CatalogPage';
import * as s from './App.css';

const SUGGESTIONS = [
  'Looking for a lead product designer',
  'I need a Rust developer with cloud expertise',
  'UI designer who has experience with Figma',
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
          hourly_rate_min: ac.hourly_rate_min,
          hourly_rate_max: ac.hourly_rate_max,
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
            <input
              className={s.searchInput}
              value={prompt}
              onChange={(e) => setPrompt(e.target.value)}
              placeholder="Describe the talent you're looking for…"
              disabled={loading}
            />
          </div>
          <button className={s.searchButton} type="submit" disabled={loading || !prompt.trim()}>
            <Sparkles className={s.buttonIcon} aria-hidden="true" />
            <span>{loading ? 'Searching…' : 'Search with AI'}</span>
          </button>
        </form>

        <div className={s.suggestions}>
          {SUGGESTIONS.map((label) => (
            <button
              key={label}
              className={s.suggestionChip}
              type="button"
              onClick={() => setPrompt(label)}
            >
              {label}
            </button>
          ))}
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
