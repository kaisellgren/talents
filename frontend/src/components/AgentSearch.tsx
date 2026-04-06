import { useState } from 'react';
import { runAgent } from '../api';
import type { AgentResponse, Candidate } from '../types';
import { CandidateCard } from './CandidateCard';
import * as s from '../styles.css';

export function AgentSearch() {
  const [prompt, setPrompt] = useState('');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<AgentResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!prompt.trim()) return;
    setLoading(true);
    setError(null);
    setResult(null);
    try {
      const res = await runAgent(prompt.trim());
      setResult(res);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className={s.page}>
      <form onSubmit={handleSubmit} className={s.page}>
        <label className={s.label}>
          Describe the candidate you're looking for
          <textarea
            className={s.textarea}
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            placeholder="e.g. Senior backend engineer in Berlin with Rust and PostgreSQL experience"
          />
        </label>
        <button className={s.button} type="submit" disabled={loading || !prompt.trim()}>
          {loading ? 'Searching…' : 'Search with AI'}
        </button>
      </form>

      {loading && <p className={s.spinner}>Running agentic loop…</p>}
      {error && <p className={s.errorMsg}>{error}</p>}

      {result && (
        <>
          <p className={s.iterNote}>Found in {result.iterations} iteration(s)</p>
          {result.candidates.length === 0 ? (
            <p className={s.empty}>No candidates matched your criteria.</p>
          ) : (
            <div className={s.grid}>
              {result.candidates.map((ac) => {
                const stub: Candidate = {
                  id: ac.id,
                  name: ac.name,
                  skills: [],
                  location_city: '',
                  location_country: '',
                  role: null,
                  available: true,
                  hourly_rate_min: null,
                  hourly_rate_max: null,
                  biography: null,
                  created_at: '',
                };
                return <CandidateCard key={ac.id} candidate={stub} agent={ac} />;
              })}
            </div>
          )}
        </>
      )}
    </div>
  );
}
