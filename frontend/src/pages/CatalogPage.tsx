import { useEffect, useState } from 'react';
import { listAvailable } from '../api';
import type { Talent } from '../types';
import { TalentCard } from '../components/TalentCard';
import * as s from './CatalogPage.css';

const PAGE_SIZE = 30;

export function CatalogPage() {
  const [talents, setTalents] = useState<Talent[]>([]);
  const [loading, setLoading] = useState(true);
  const [allLoaded, setAllLoaded] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadMore(0);
  }, []);

  async function loadMore(offset: number) {
    setLoading(true);
    try {
      const page = await listAvailable(PAGE_SIZE, offset);
      setTalents((prev) => (offset === 0 ? page : [...prev, ...page]));
      if (page.length < PAGE_SIZE) setAllLoaded(true);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load talents');
    } finally {
      setLoading(false);
    }
  }

  return (
    <section className={s.catalogSection}>
      <div className={s.catalogHeader}>
        <h2 className={s.catalogTitle}>Available Talents</h2>
      </div>

      {error && <p className={s.errorMsg}>{error}</p>}

      {talents.length === 0 && loading ? (
        <p className={s.loadingMsg}>Loading talents…</p>
      ) : (
        <div className={s.grid}>
          {talents.map((t) => (
            <TalentCard key={t.id} talent={t} />
          ))}
        </div>
      )}

      {!allLoaded && !error && (
        <div className={s.viewMoreRow}>
          <button
            className={s.viewMoreBtn}
            onClick={() => loadMore(talents.length)}
            disabled={loading}
          >
            {loading && talents.length > 0 ? 'Loading…' : 'View More Candidates'}
          </button>
        </div>
      )}
    </section>
  );
}
