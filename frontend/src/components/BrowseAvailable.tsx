import { useEffect, useState } from 'react';
import { listAvailable } from '../api';
import type { Candidate } from '../types';
import { CandidateCard } from './CandidateCard';
import * as s from '../styles.css';

export function BrowseAvailable() {
  const [candidates, setCandidates] = useState<Candidate[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    listAvailable()
      .then(setCandidates)
      .catch((err) => setError(err instanceof Error ? err.message : 'Unknown error'))
      .finally(() => setLoading(false));
  }, []);

  if (loading) return <p className={s.spinner}>Loading candidates…</p>;
  if (error) return <p className={s.errorMsg}>{error}</p>;
  if (candidates.length === 0) return <p className={s.empty}>No available candidates.</p>;

  return (
    <div className={s.grid}>
      {candidates.map((c) => (
        <CandidateCard key={c.id} candidate={c} />
      ))}
    </div>
  );
}
