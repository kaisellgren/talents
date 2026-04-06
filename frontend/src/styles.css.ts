import { style } from '@vanilla-extract/css';

export const page = style({
  display: 'flex',
  flexDirection: 'column',
  gap: '16px',
});

export const grid = style({
  display: 'grid',
  gridTemplateColumns: 'repeat(auto-fill, minmax(320px, 1fr))',
  gap: '16px',
});

export const label = style({
  display: 'flex',
  flexDirection: 'column',
  gap: '6px',
  fontSize: '0.9rem',
  color: '#aaaacc',
});

export const input = style({
  background: '#1a1a2e',
  border: '1px solid #2a2a4a',
  borderRadius: '6px',
  padding: '8px 12px',
  color: '#e0e0ff',
  fontSize: '0.95rem',
  outline: 'none',
  ':focus': {
    borderColor: '#5555cc',
  },
});

export const textarea = style({
  background: '#1a1a2e',
  border: '1px solid #2a2a4a',
  borderRadius: '6px',
  padding: '10px 12px',
  color: '#e0e0ff',
  fontSize: '0.95rem',
  outline: 'none',
  resize: 'vertical',
  minHeight: '100px',
  fontFamily: 'inherit',
  ':focus': {
    borderColor: '#5555cc',
  },
});

export const button = style({
  background: '#3a3aaa',
  color: '#ffffff',
  border: 'none',
  borderRadius: '6px',
  padding: '10px 20px',
  fontSize: '0.95rem',
  fontWeight: 600,
  cursor: 'pointer',
  alignSelf: 'flex-start',
  ':hover': {
    background: '#4a4acc',
  },
  ':disabled': {
    opacity: 0.5,
    cursor: 'not-allowed',
  },
});

export const errorMsg = style({
  color: '#ff6666',
  fontSize: '0.9rem',
});

export const spinner = style({
  color: '#7777cc',
  fontSize: '0.9rem',
});

export const empty = style({
  color: '#666688',
  fontSize: '0.9rem',
});

export const iterNote = style({
  color: '#7777aa',
  fontSize: '0.85rem',
});

export const row = style({
  display: 'flex',
  gap: '12px',
  flexWrap: 'wrap',
});
