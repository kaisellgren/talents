import { style, keyframes } from '@vanilla-extract/css';

const pulse = keyframes({
  '0%, 100%': { opacity: 0.4 },
  '60%': { opacity: 1 },
});

export const root = style({
  minHeight: '100vh',
  background: '#060e20',
  color: '#dee5ff',
  fontFamily: "'Inter', system-ui, sans-serif",
  display: 'flex',
  flexDirection: 'column',
  position: 'relative',
});

export const glowTopRight = style({
  position: 'fixed',
  top: '-200px',
  right: '-200px',
  width: '600px',
  height: '600px',
  borderRadius: '50%',
  background: 'radial-gradient(circle, rgba(163, 166, 255, 0.09) 0%, transparent 70%)',
  pointerEvents: 'none',
  zIndex: 0,
});

export const glowBottomLeft = style({
  position: 'fixed',
  bottom: '-250px',
  left: '-150px',
  width: '500px',
  height: '500px',
  borderRadius: '50%',
  background: 'radial-gradient(circle, rgba(147, 150, 255, 0.05) 0%, transparent 70%)',
  pointerEvents: 'none',
  zIndex: 0,
});

export const header = style({
  position: 'sticky',
  top: 0,
  zIndex: 100,
  background: 'rgba(6, 14, 32, 0.85)',
  backdropFilter: 'blur(20px)',
  WebkitBackdropFilter: 'blur(20px)',
  borderBottom: '1px solid rgba(163, 166, 255, 0.08)',
  padding: '0 48px',
  height: '60px',
  display: 'flex',
  alignItems: 'center',
  gap: '40px',
});

export const title = style({
  fontFamily: "'Manrope', sans-serif",
  fontSize: '1rem',
  fontWeight: 800,
  background: 'linear-gradient(135deg, #a3a6ff 0%, #9396ff 100%)',
  WebkitBackgroundClip: 'text',
  WebkitTextFillColor: 'transparent',
  backgroundClip: 'text',
  margin: 0,
  letterSpacing: '-0.02em',
  flexShrink: 0,
});

export const nav = style({
  display: 'flex',
  gap: '1rem',
  margin: '0 auto',
});

export const navLink = style({
  color: '#6d758c',
  textDecoration: 'none',
  fontSize: '0.875rem',
  fontWeight: 500,
  padding: '6px 10px',
  transition: 'color 0.15s ease',
  ':hover': {
    color: '#a3aac4',
  },
});

export const navLinkActive = style({
  color: '#a3a6ff',
  textDecoration: 'none',
  fontSize: '0.875rem',
  fontWeight: 500,
  padding: '6px 10px',
  borderBottom: '1px solid #a3a6ff',
});

export const navLinkContent = style({
  display: 'inline-flex',
  alignItems: 'center',
  gap: '8px',
});

export const navIcon = style({
  width: '15px',
  height: '15px',
  flexShrink: 0,
});

/* ── Logo ── */

export const logoMark = style({
  width: '52px',
  height: '52px',
  marginBottom: '28px',
  filter: 'drop-shadow(0 0 18px rgba(163, 166, 255, 0.45)) drop-shadow(0 0 6px rgba(163, 166, 255, 0.25))',
});

/* ── Hero ── */

export const heroSection = style({
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  textAlign: 'center',
  padding: '96px 48px 80px',
  position: 'relative',
  zIndex: 1,
});

export const heroEyebrow = style({
  fontFamily: "'Inter', sans-serif",
  fontSize: '0.72rem',
  fontWeight: 600,
  letterSpacing: '0.12em',
  textTransform: 'uppercase',
  color: '#a3a6ff',
  margin: '0 0 20px',
});

export const heroTitle = style({
  fontFamily: "'Manrope', sans-serif",
  fontSize: 'clamp(4rem, 8vw, 7rem)',
  fontWeight: 800,
  letterSpacing: '-0.04em',
  lineHeight: 1,
  margin: '0 0 24px',
  background: 'linear-gradient(160deg, #dee5ff 30%, rgba(163, 166, 255, 0.6) 100%)',
  WebkitBackgroundClip: 'text',
  WebkitTextFillColor: 'transparent',
  backgroundClip: 'text',
  textShadow: 'none',
});

export const heroSubtitle = style({
  fontSize: '1.2rem',
  color: '#a3aac4',
  maxWidth: '800px',
  lineHeight: 1.65,
  margin: '0 0 40px',
});

export const heroNote = style({
  color: '#403b7e',
  fontStyle: 'italic',
});

export const searchForm = style({
  display: 'flex',
  gap: '10px',
  alignItems: 'stretch',
  width: '100%',
  maxWidth: '800px',
  margin: '0 0 20px',
});

export const searchInputShell = style({
  position: 'relative',
  flex: 1,
  display: 'flex',
  alignItems: 'center',
  background: 'rgba(15, 25, 48, 0.8)',
  border: '1px solid rgba(163, 166, 255, 0.15)',
  borderRadius: '10px',
  transition: 'border-color 0.15s ease, background 0.15s ease',
  ':focus-within': {
    borderColor: 'rgba(163, 166, 255, 0.4)',
    background: 'rgba(20, 31, 56, 0.9)',
  },
});

export const searchInputIcon = style({
  position: 'absolute',
  top: '35%',
  left: '16px',
  transform: 'translateY(-50%)',
  width: '18px',
  height: '18px',
  color: '#6d758c',
  pointerEvents: 'none',
  zIndex: 1,
});

export const searchInput = style({
  width: '100%',
  background: 'transparent',
  border: 'none',
  borderRadius: '10px',
  padding: '13px 18px 13px 46px',
  color: '#dee5ff',
  fontSize: '1.05rem',
  outline: 'none',
  fontFamily: "'Inter', sans-serif",
  '::placeholder': {
    color: '#8891a8',
  },
  ':disabled': {
    opacity: 0.6,
  },
  overflow: 'hidden',
  overflowWrap: 'break-word',
  whiteSpace: 'pre-wrap',
});

export const searchButton = style({
  background: 'linear-gradient(135deg, #a3a6ff 0%, #9396ff 100%)',
  color: '#0a0081',
  border: 'none',
  borderRadius: '10px',
  padding: '13px 22px',
  fontSize: '1rem',
  fontWeight: 700,
  cursor: 'pointer',
  fontFamily: "'Inter', sans-serif",
  display: 'inline-flex',
  alignItems: 'center',
  gap: '8px',
  whiteSpace: 'nowrap',
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

export const buttonIcon = style({
  width: '16px',
  height: '16px',
  flexShrink: 0,
});

export const promptCloud = style({
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  gap: '14px',
  margin: '4px 0 28px',
  maxWidth: '720px',
  width: '100%',
});

export const promptCloudLabel = style({
  fontSize: '0.68rem',
  fontWeight: 700,
  letterSpacing: '0.12em',
  textTransform: 'uppercase',
  color: '#40485d',
  margin: 0,
  fontFamily: "'Inter', sans-serif",
});

export const promptChips = style({
  display: 'flex',
  flexWrap: 'wrap',
  gap: '8px',
  justifyContent: 'center',
});

export const promptChip = style({
  background: 'rgba(25, 37, 64, 0.5)',
  border: '1px solid rgba(163, 166, 255, 0.1)',
  borderRadius: '999px',
  padding: '5px 13px',
  fontSize: '0.82rem',
  color: '#8891a8',
  cursor: 'pointer',
  fontFamily: "'Inter', sans-serif",
  transition: 'all 0.15s ease',
  ':hover': {
    color: '#dee5ff',
    borderColor: 'rgba(163, 166, 255, 0.3)',
    background: 'rgba(163, 166, 255, 0.08)',
  },
});

export const statusMsg = style({
  display: 'flex',
  alignItems: 'center',
  gap: '8px',
  color: '#6d758c',
  fontSize: '0.82rem',
  fontStyle: 'italic',
  fontFamily: "'Inter', sans-serif",
});

export const statusDot = style({
  width: '6px',
  height: '6px',
  borderRadius: '50%',
  background: '#a3a6ff',
  flexShrink: 0,
  animation: `${pulse} 1.6s cubic-bezier(0.4, 0, 0.6, 1) infinite`,
});

export const errorMsg = style({
  color: '#ff6e84',
  fontSize: '0.875rem',
  marginTop: '8px',
});

/* ── Catalog ── */

export const catalogSection = style({
  position: 'relative',
  zIndex: 1,
  padding: '0 48px 80px',
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
});
