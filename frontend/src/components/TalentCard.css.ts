import { style } from '@vanilla-extract/css';

export const card = style({
  background: '#1a1a2e',
  border: '1px solid #2a2a4a',
  borderRadius: '10px',
  padding: '20px',
  display: 'flex',
  flexDirection: 'column',
  gap: '10px',
});

export const name = style({
  fontSize: '1.1rem',
  fontWeight: 700,
  color: '#e0e0ff',
  margin: 0,
});

export const meta = style({
  fontSize: '0.85rem',
  color: '#8888aa',
  margin: 0,
});

export const skillList = style({
  display: 'flex',
  flexWrap: 'wrap',
  gap: '6px',
});

export const skill = style({
  background: '#2a2a4a',
  color: '#a0a0dd',
  borderRadius: '4px',
  padding: '2px 8px',
  fontSize: '0.78rem',
});

export const bio = style({
  fontSize: '0.88rem',
  color: '#aaaacc',
  margin: 0,
  lineHeight: 1.5,
});

export const score = style({
  fontSize: '0.9rem',
  fontWeight: 600,
  color: '#7c7cff',
});

export const reasoning = style({
  fontSize: '0.85rem',
  color: '#9999bb',
  fontStyle: 'italic',
  margin: 0,
  lineHeight: 1.5,
});

export const summary = style({
  fontSize: '0.88rem',
  color: '#bbbbdd',
  margin: 0,
  lineHeight: 1.5,
});

export const rate = style({
  fontSize: '0.85rem',
  color: '#88ddaa',
});
