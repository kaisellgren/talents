import { useState } from 'react';
import { searchTalents } from '../api';
import type { Talent } from '../types';
import { TalentCard } from './TalentCard';
import * as s from '../styles.css';

export function SkillSearch() {
  const [skillsInput, setSkillsInput] = useState('');
  const [city, setCity] = useState('');
  const [country, setCountry] = useState('');
  const [talents, setTalents] = useState<Talent[] | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    const skills = skillsInput
      .split(',')
      .map((s) => s.trim())
      .filter(Boolean);
    if (skills.length === 0) return;
    setLoading(true);
    setError(null);
    try {
      const res = await searchTalents(skills, city.trim() || undefined, country.trim() || undefined);
      setTalents(res);
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
          Skills (comma-separated)
          <input
            className={s.input}
            value={skillsInput}
            onChange={(e) => setSkillsInput(e.target.value)}
            placeholder="e.g. Rust, PostgreSQL, Docker"
          />
        </label>
        <div className={s.row}>
          <label className={s.label} style={{ flex: 1 }}>
            City (optional)
            <input
              className={s.input}
              value={city}
              onChange={(e) => setCity(e.target.value)}
              placeholder="Berlin"
            />
          </label>
          <label className={s.label} style={{ flex: 1 }}>
            Country (optional)
            <input
              className={s.input}
              value={country}
              onChange={(e) => setCountry(e.target.value)}
              placeholder="Germany"
            />
          </label>
        </div>
        <button className={s.button} type="submit" disabled={loading || !skillsInput.trim()}>
          {loading ? 'Searching…' : 'Search'}
        </button>
      </form>

      {error && <p className={s.errorMsg}>{error}</p>}

      {talents !== null && (
        talents.length === 0
          ? <p className={s.empty}>No talents matched your search.</p>
          : (
            <div className={s.grid}>
              {talents.map((c) => (
                <TalentCard key={c.id} talent={c} />
              ))}
            </div>
          )
      )}
    </div>
  );
}
