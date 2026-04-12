import { useEffect, useState } from 'react';
import * as s from './AgentLoader.css';

const STEPS = [
  'Analyzing your requirements',
  'Searching the talent database',
  'Applying filters and constraints',
  'Ranking candidates by fit',
  'Generating candidate summaries',
];

export function AgentLoader() {
  const [activeStep, setActiveStep] = useState(0);

  useEffect(() => {
    const id = setInterval(() => {
      setActiveStep(prev => Math.min(prev + 1, STEPS.length - 1));
    }, 2800);
    return () => clearInterval(id);
  }, []);

  return (
    <div className={s.wrapper}>
      <div className={s.card}>
        <div className={s.orbWrap}>
          <div className={s.ring} />
          <div className={s.ring2} />
          <div className={s.ring3} />
          <div className={s.core} />
        </div>

        <div className={s.headerBlock}>
          <p className={s.label}>AI agents at work</p>
          <p className={s.sublabel}>Running multi-step agentic pipeline</p>
        </div>

        <div className={s.shimmerBar} />

        <ul className={s.steps}>
          {STEPS.map((step, i) => {
            const state = i < activeStep ? 'done' : i === activeStep ? 'active' : 'pending';
            return (
              <li
                key={step}
                className={state === 'done' ? s.stepDone : state === 'active' ? s.stepActive : s.stepPending}
              >
                <span className={state === 'done' ? s.dotDone : state === 'active' ? s.dotActive : s.dotPending} />
                {step}
              </li>
            );
          })}
        </ul>
      </div>
    </div>
  );
}
