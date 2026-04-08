import type { AgentTalent, Talent } from '../types';
import { getSkillIcon } from '../skillIcons';
import * as s from './TalentCard.css';

interface TalentCardProps {
  talent: Talent;
  agent?: AgentTalent;
}

export function TalentCard({ talent, agent }: TalentCardProps) {
  const rateStr =
    talent.hourly_rate_min != null && talent.hourly_rate_max != null
      ? `${talent.hourly_rate_min}–${talent.hourly_rate_max} €/hr`
      : talent.hourly_rate_min != null
        ? `from ${talent.hourly_rate_min} €/hr`
        : null;

  const scoreDisplay = agent ? `${(agent.score * 100).toFixed(0)}% match` : null;
  const hasLocation = talent.location_city || talent.location_country;
  const locationStr = [talent.location_city, talent.location_country].filter(Boolean).join(', ');

  return (
    <div className={agent ? s.cardAgent : s.card}>
      {/* Header: name/role/location + score/rate chips */}
      <div className={s.cardHeader}>
        <div className={s.nameBlock}>
          <p className={s.name}>{talent.name}</p>
          {talent.role && <p className={s.role}>{talent.role}</p>}
          {hasLocation && (
            <p className={s.location}>
              <svg width="11" height="13" viewBox="0 0 12 14" fill="none" aria-hidden="true">
                <path d="M6 0C3.24 0 1 2.24 1 5c0 3.75 5 9 5 9s5-5.25 5-9c0-2.76-2.24-5-5-5zm0 6.5a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3z" fill="#6d758c"/>
              </svg>
              {locationStr}
            </p>
          )}
        </div>
        <div className={s.chips}>
          {scoreDisplay && <span className={agent ? s.scoreChipAgent : s.scoreChip}>{scoreDisplay}</span>}
          {rateStr && <span className={s.rateChip}>{rateStr}</span>}
        </div>
      </div>

      {/* Biography */}
      {talent.biography && (
        <p className={s.bio}>{talent.biography}</p>
      )}

      {/* Skills */}
      {talent.skills.length > 0 && (
        <div className={s.skillList}>
          {talent.skills.map((sk) => {
            const iconEntry = getSkillIcon(sk);
            return (
              <span key={sk} className={s.skill}>
                {iconEntry?.type === 'simple' && (
                  <svg
                    className={s.skillIcon}
                    role="img"
                    viewBox="0 0 24 24"
                    aria-hidden="true"
                    fill="currentColor"
                  >
                    <path d={iconEntry.icon.path} />
                  </svg>
                )}
                {iconEntry?.type === 'lucide' && (
                  <iconEntry.icon className={s.skillIcon} aria-hidden="true" />
                )}
                {sk}
              </span>
            );
          })}
        </div>
      )}

      <div className={s.flex1}></div>

      {/* AI Match Summary (between skills and bio, shown when agent result) */}
      {agent && (agent.summary || agent.reasoning) && (
        <div className={s.aiSection}>
          <div className={s.aiOrb} />
          <div className={s.aiMatchHeader}>
            <div className={s.aiIconWrap}>
              <svg className={s.aiIcon} width="14" height="14" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                <path d="M19 9l1.25-2.75L23 5l-2.75-1.25L19 1l-1.25 2.75L15 5l2.75 1.25zM11.5 9.5L9 4 6.5 9.5 1 12l5.5 2.5L9 20l2.5-5.5L17 12zM19 15l-1.25 2.75L15 19l2.75 1.25L19 23l1.25-2.75L23 19l-2.75-1.25z"/>
              </svg>
            </div>
            <span className={s.aiMatchLabel}>AI Match Summary</span>
          </div>
          {agent.summary && <p className={s.summary}>{agent.summary}</p>}
          {agent.reasoning && <p className={s.reasoning}>{agent.reasoning}</p>}
        </div>
      )}
    </div>
  );
}
