import type { AgentCandidate, Candidate } from '../types';
import * as s from './CandidateCard.css';

interface CandidateCardProps {
  candidate: Candidate;
  agent?: AgentCandidate;
}

export function CandidateCard({ candidate, agent }: CandidateCardProps) {
  const rateStr =
    candidate.hourly_rate_min != null && candidate.hourly_rate_max != null
      ? `$${candidate.hourly_rate_min}–$${candidate.hourly_rate_max}/hr`
      : candidate.hourly_rate_min != null
        ? `from $${candidate.hourly_rate_min}/hr`
        : null;

  return (
    <div className={s.card}>
      <p className={s.name}>{candidate.name}</p>
      <p className={s.meta}>
        {candidate.location_city}, {candidate.location_country}
        {candidate.role ? ` · ${candidate.role}` : ''}
      </p>
      {rateStr && <span className={s.rate}>{rateStr}</span>}
      {candidate.skills.length > 0 && (
        <div className={s.skillList}>
          {candidate.skills.map((sk) => (
            <span key={sk} className={s.skill}>{sk}</span>
          ))}
        </div>
      )}
      {agent && (
        <>
          <span className={s.score}>Score: {agent.score.toFixed(2)}</span>
          {agent.summary && <p className={s.summary}>{agent.summary}</p>}
          {agent.reasoning && <p className={s.reasoning}>{agent.reasoning}</p>}
        </>
      )}
      {!agent && candidate.biography && (
        <p className={s.bio}>{candidate.biography}</p>
      )}
    </div>
  );
}
