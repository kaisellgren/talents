import { style } from '@vanilla-extract/css';

export const catalogSection = style({
  position: 'relative',
  zIndex: 1,
  padding: '48px',
  maxWidth: '1280px',
  width: '100%',
  margin: '0 auto',
  boxSizing: 'border-box',
});

export const catalogHeader = style({
  marginBottom: '24px',
  borderBottom: '1px solid rgba(163, 166, 255, 0.07)',
  paddingBottom: '20px',
});

export const catalogTitle = style({
  fontFamily: "'Manrope', sans-serif",
  fontSize: '1.1rem',
  fontWeight: 700,
  color: '#dee5ff',
  letterSpacing: '-0.02em',
  margin: 0,
});

export const grid = style({
  display: 'grid',
  gridTemplateColumns: 'repeat(3, 1fr)',
  gap: '16px',
  '@media': {
    'screen and (max-width: 1100px)': {
      gridTemplateColumns: 'repeat(2, 1fr)',
    },
    'screen and (max-width: 680px)': {
      gridTemplateColumns: '1fr',
    },
  },
});

export const loadingMsg = style({
  color: '#40485d',
  fontSize: '0.875rem',
  fontFamily: "'Inter', sans-serif",
});

export const errorMsg = style({
  color: '#ff6e84',
  fontSize: '0.875rem',
  fontFamily: "'Inter', sans-serif",
});

export const viewMoreRow = style({
  display: 'flex',
  justifyContent: 'center',
  marginTop: '40px',
});

export const viewMoreBtn = style({
  background: 'transparent',
  border: '1px solid rgba(163, 166, 255, 0.18)',
  borderRadius: '10px',
  padding: '11px 28px',
  color: '#a3aac4',
  fontSize: '0.875rem',
  fontWeight: 500,
  cursor: 'pointer',
  fontFamily: "'Inter', sans-serif",
  transition: 'all 0.15s ease',
  ':hover': {
    background: 'rgba(163, 166, 255, 0.06)',
    color: '#dee5ff',
    borderColor: 'rgba(163, 166, 255, 0.3)',
  },
  ':disabled': {
    opacity: 0.5,
    cursor: 'not-allowed',
  },
});
