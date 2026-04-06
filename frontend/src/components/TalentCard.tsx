import type { AgentTalent, Talent } from '../types';
import * as s from './TalentCard.css';

interface TalentCardProps {
  talent: Talent;
  agent?: AgentTalent;
}

export function TalentCard({ talent, agent }: TalentCardProps) {
  const rateStr =
    talent.hourly_rate_min != null && talent.hourly_rate_max != null
      ? `$${talent.hourly_rate_min}–$${talent.hourly_rate_max}/hr`
      : talent.hourly_rate_min != null
        ? `from $${talent.hourly_rate_min}/hr`
        : null;

  return (
    <div className={s.card}>
      <p className={s.name}>{talent.name}</p>
      <p className={s.meta}>
        {talent.location_city}, {talent.location_country}
        {talent.role ? ` · ${talent.role}` : ''}
      </p>
      {rateStr && <span className={s.rate}>{rateStr}</span>}
      {talent.skills.length > 0 && (
        <div className={s.skillList}>
          {talent.skills.map((sk) => (
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
      {!agent && talent.biography && (
        <p className={s.bio}>{talent.biography}</p>
      )}
    </div>
  );
}
