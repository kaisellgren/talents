import { style, styleVariants } from '@vanilla-extract/css';

export const root = style({
  minHeight: '100vh',
  background: '#0d0d1a',
  color: '#e0e0ff',
  fontFamily: 'system-ui, sans-serif',
  display: 'flex',
  flexDirection: 'column',
});

export const header = style({
  borderBottom: '1px solid #1e1e3a',
  padding: '16px 32px',
  display: 'flex',
  alignItems: 'center',
  gap: '32px',
});

export const title = style({
  fontSize: '1.2rem',
  fontWeight: 700,
  color: '#9090ff',
  margin: 0,
  whiteSpace: 'nowrap',
});

export const tabs = style({
  display: 'flex',
  gap: '4px',
});

const tabBase = style({
  background: 'transparent',
  border: 'none',
  borderRadius: '6px',
  padding: '8px 16px',
  fontSize: '0.9rem',
  cursor: 'pointer',
  transition: 'background 0.15s',
});

export const tab = styleVariants({
  inactive: [tabBase, { color: '#7777aa', ':hover': { background: '#1a1a2e' } }],
  active: [tabBase, { color: '#e0e0ff', background: '#1e1e3a' }],
});

export const content = style({
  padding: '28px 32px',
  flex: 1,
  maxWidth: '1100px',
  width: '100%',
  margin: '0 auto',
  boxSizing: 'border-box',
});
