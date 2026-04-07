import { style } from '@vanilla-extract/css';

export const page = style({
  display: 'flex',
  flexDirection: 'column',
  gap: '20px',
});

export const grid = style({
  display: 'grid',
  gridTemplateColumns: 'repeat(auto-fill, minmax(360px, 1fr))',
  gap: '16px',
});

export const label = style({
  display: 'flex',
  flexDirection: 'column',
  gap: '8px',
  fontSize: '0.82rem',
  fontWeight: 500,
  color: '#6d758c',
  letterSpacing: '0.03em',
  textTransform: 'uppercase',
});

export const input = style({
  background: 'rgba(9, 19, 40, 0.7)',
  border: '1px solid rgba(163, 166, 255, 0.12)',
  borderRadius: '10px',
  padding: '11px 16px',
  color: '#dee5ff',
  fontSize: '0.9rem',
  outline: 'none',
  fontFamily: "'Inter', sans-serif",
  transition: 'border-color 0.15s ease, background 0.15s ease',
  ':focus': {
    borderColor: 'rgba(163, 166, 255, 0.4)',
    background: 'rgba(15, 25, 48, 0.9)',
  },
  '::placeholder': {
    color: '#40485d',
  },
});

export const textarea = style({
  background: 'rgba(9, 19, 40, 0.7)',
  border: '1px solid rgba(163, 166, 255, 0.12)',
  borderRadius: '10px',
  padding: '14px 16px',
  color: '#dee5ff',
  fontSize: '0.9rem',
  outline: 'none',
  resize: 'vertical',
  minHeight: '120px',
  fontFamily: "'Inter', sans-serif",
  lineHeight: 1.6,
  transition: 'border-color 0.15s ease, background 0.15s ease',
  ':focus': {
    borderColor: 'rgba(163, 166, 255, 0.4)',
    background: 'rgba(15, 25, 48, 0.9)',
  },
  '::placeholder': {
    color: '#40485d',
  },
});

export const button = style({
  background: 'linear-gradient(135deg, #a3a6ff 0%, #9396ff 100%)',
  color: '#0f00a4',
  border: 'none',
  borderRadius: '10px',
  padding: '11px 24px',
  fontSize: '0.875rem',
  fontWeight: 700,
  cursor: 'pointer',
  alignSelf: 'flex-start',
  fontFamily: "'Inter', sans-serif",
  letterSpacing: '0.01em',
  transition: 'opacity 0.15s ease, transform 0.15s ease',
  ':hover': {
    opacity: 0.9,
    transform: 'translateY(-1px)',
  },
  ':disabled': {
    opacity: 0.35,
    cursor: 'not-allowed',
    transform: 'none',
  },
});

export const errorMsg = style({
  color: '#ff6e84',
  fontSize: '0.875rem',
  fontFamily: "'Inter', sans-serif",
});

export const spinner = style({
  color: '#6d758c',
  fontSize: '0.875rem',
  fontFamily: "'Inter', sans-serif",
});

export const empty = style({
  color: '#40485d',
  fontSize: '0.875rem',
  fontFamily: "'Inter', sans-serif",
});

export const iterNote = style({
  color: '#6d758c',
  fontSize: '0.8rem',
  fontFamily: "'Inter', sans-serif",
  letterSpacing: '0.02em',
});

export const row = style({
  display: 'flex',
  gap: '12px',
  flexWrap: 'wrap',
});
