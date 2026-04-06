import { useState } from 'react';
import { AgentSearch } from './components/AgentSearch';
import { BrowseAvailable } from './components/BrowseAvailable';
import { SkillSearch } from './components/SkillSearch';
import * as s from './App.css';

type Tab = 'agent' | 'browse' | 'search';

const TABS: { id: Tab; label: string }[] = [
  { id: 'agent', label: 'AI Search' },
  { id: 'browse', label: 'Browse Available' },
  { id: 'search', label: 'Skill Search' },
];

function App() {
  const [active, setActive] = useState<Tab>('agent');

  return (
    <div className={s.root}>
      <header className={s.header}>
        <p className={s.title}>Talent Platform</p>
        <nav className={s.tabs}>
          {TABS.map(({ id, label }) => (
            <button
              key={id}
              className={s.tab[active === id ? 'active' : 'inactive']}
              onClick={() => setActive(id)}
            >
              {label}
            </button>
          ))}
        </nav>
      </header>
      <main className={s.content}>
        {active === 'agent' && <AgentSearch />}
        {active === 'browse' && <BrowseAvailable />}
        {active === 'search' && <SkillSearch />}
      </main>
    </div>
  );
}

export default App;
