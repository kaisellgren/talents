import { useEffect, useState } from 'react';
import { listAvailable } from '../api';
import type { Talent } from '../types';
import { TalentCard } from './TalentCard';
import * as s from '../styles.css';

export function BrowseAvailable() {
  const [talents, setTalents] = useState<Talent[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    listAvailable()
      .then(setTalents)
      .catch((err) => setError(err instanceof Error ? err.message : 'Unknown error'))
      .finally(() => setLoading(false));
  }, []);

  if (loading) return <p className={s.spinner}>Loading talents…</p>;
  if (error) return <p className={s.errorMsg}>{error}</p>;
  if (talents.length === 0) return <p className={s.empty}>No available talents.</p>;

  return (
    <div className={s.grid}>
      {talents.map((c) => (
        <TalentCard key={c.id} talent={c} />
      ))}
    </div>
  );
}
