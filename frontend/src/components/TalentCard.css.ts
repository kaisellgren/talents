import { style } from '@vanilla-extract/css';

export const card = style({
  background: 'rgba(9, 19, 40, 0.7)',
  backdropFilter: 'blur(20px)',
  WebkitBackdropFilter: 'blur(20px)',
  border: '1px solid rgba(163, 166, 255, 0.1)',
  borderRadius: '14px',
  padding: '24px',
  display: 'flex',
  flexDirection: 'column',
  gap: '14px',
  position: 'relative',
  transition: 'border-color 0.2s ease, transform 0.2s ease',
  ':hover': {
    borderColor: 'rgba(163, 166, 255, 0.25)',
    transform: 'translateY(-2px)',
  },
});

export const cardAgent = style({
  background: 'rgba(30, 10, 35, 0.75)',
  backdropFilter: 'blur(20px)',
  WebkitBackdropFilter: 'blur(20px)',
  border: '1px solid rgba(255, 142, 210, 0.22)',
  borderRadius: '14px',
  padding: '24px',
  display: 'flex',
  flexDirection: 'column',
  gap: '14px',
  position: 'relative',
  transition: 'border-color 0.2s ease, transform 0.2s ease',
  boxShadow: '0 0 18px rgba(255, 100, 180, 0.06), inset 0 0 30px rgba(255, 100, 180, 0.03)',
  ':hover': {
    borderColor: 'rgba(255, 142, 210, 0.42)',
    transform: 'translateY(-2px)',
    boxShadow: '0 0 28px rgba(255, 100, 180, 0.12), inset 0 0 30px rgba(255, 100, 180, 0.05)',
  },
});

export const cardHeader = style({
  display: 'flex',
  alignItems: 'flex-start',
  justifyContent: 'space-between',
  gap: '16px',
});

export const nameBlock = style({
  flex: 1,
  minWidth: 0,
});

export const name = style({
  fontFamily: "'Manrope', sans-serif",
  fontSize: '1.15rem',
  fontWeight: 700,
  color: '#dee5ff',
  margin: 0,
  lineHeight: 1.3,
  letterSpacing: '-0.01em',
});

export const role = style({
  fontSize: '0.9rem',
  color: '#a3aac4',
  margin: '3px 0 0',
  fontFamily: "'Inter', sans-serif",
});

export const location = style({
  fontSize: '0.78rem',
  color: '#6d758c',
  margin: '6px 0 0',
  display: 'flex',
  alignItems: 'center',
  gap: '4px',
});

export const chips = style({
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'flex-end',
  gap: '6px',
  flexShrink: 0,
});

export const scoreChipAgent = style({
  background: 'rgba(255, 142, 210, 0.12)',
  border: '1px solid rgba(255, 142, 210, 0.3)',
  color: '#ffa5d9',
  borderRadius: '6px',
  padding: '4px 10px',
  fontSize: '0.88rem',
  fontWeight: 700,
  fontFamily: "'Manrope', sans-serif",
  whiteSpace: 'nowrap',
  letterSpacing: '0.01em',
});

export const scoreChip = style({
  background: 'rgba(163, 166, 255, 0.12)',
  border: '1px solid rgba(163, 166, 255, 0.25)',
  color: '#a3a6ff',
  borderRadius: '6px',
  padding: '4px 10px',
  fontSize: '0.78rem',
  fontWeight: 700,
  fontFamily: "'Manrope', sans-serif",
  whiteSpace: 'nowrap',
  letterSpacing: '0.01em',
});

export const rateChip = style({
  background: 'rgba(86, 190, 140, 0.08)',
  border: '1px solid rgba(86, 190, 140, 0.2)',
  color: '#7dd4a8',
  borderRadius: '6px',
  padding: '4px 10px',
  fontSize: '0.86rem',
  fontWeight: 600,
  whiteSpace: 'nowrap',
});

export const divider = style({
  height: '1px',
  background: 'rgba(163, 166, 255, 0.06)',
  margin: '0 -4px',
});

export const bio = style({
  fontSize: '0.94rem',
  color: '#a3aac4',
  margin: 0,
  lineHeight: 1.65,
  fontFamily: "'Inter', sans-serif",
});

export const aiSection = style({
  padding: '14px',
  borderRadius: '12px',
  background: 'linear-gradient(135deg, rgba(163,166,255,0.10) 0%, rgba(25,37,64,0.40) 55%, transparent 100%)',
  border: '1px solid rgba(163,166,255,0.20)',
  boxShadow: '0 0 15px rgba(163,166,255,0.10)',
  position: 'relative',
  overflow: 'hidden',
  display: 'flex',
  flexDirection: 'column',
  gap: '8px',
});

export const aiOrb = style({
  position: 'absolute',
  top: 0,
  right: 0,
  width: '96px',
  height: '96px',
  background: 'rgba(163,166,255,0.05)',
  borderRadius: '50%',
  transform: 'translate(50%, -50%)',
  filter: 'blur(24px)',
  pointerEvents: 'none',
});

export const aiMatchHeader = style({
  display: 'flex',
  alignItems: 'center',
  gap: '8px',
  position: 'relative',
  zIndex: 1,
});

export const aiIconWrap = style({
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  background: 'rgba(163,166,255,0.20)',
  borderRadius: '50%',
  width: '24px',
  height: '24px',
  flexShrink: 0,
});

export const aiIcon = style({
  color: '#ffa5d9',
  filter: 'drop-shadow(0 0 5px rgba(255,142,210,0.8))',
});

export const aiMatchLabel = style({
  fontSize: '14px',
  fontWeight: 900,
  letterSpacing: '0.1em',
  textTransform: 'uppercase',
  color: '#a3a6ff',
  fontFamily: "'Manrope', sans-serif",
});

export const summary = style({
  fontSize: '0.95rem',
  color: '#dee5ff',
  margin: 0,
  lineHeight: 1.65,
  fontWeight: 500,
  letterSpacing: '-0.01em',
  fontFamily: "'Inter', sans-serif",
  position: 'relative',
  zIndex: 1,
});

export const reasoning = style({
  fontSize: '0.8rem',
  color: '#8892aa',
  margin: 0,
  lineHeight: 1.55,
  fontStyle: 'italic',
  fontFamily: "'Inter', sans-serif",
  position: 'relative',
  zIndex: 1,
});

export const skillList = style({
  display: 'flex',
  flexWrap: 'wrap',
  gap: '6px',
});

export const skill = style({
  display: 'inline-flex',
  alignItems: 'center',
  gap: '5px',
  background: 'rgba(25, 37, 64, 0.8)',
  color: '#a3aac4',
  borderRadius: '5px',
  padding: '3px 9px',
  fontSize: '0.83rem',
  fontFamily: "'Inter', sans-serif",
  letterSpacing: '0.01em',
  border: '1px solid rgba(163, 166, 255, 0.07)',
});

export const skillIcon = style({
  width: '11px',
  height: '11px',
  flexShrink: 0,
  opacity: 0.7,
});

export const flex1 = style({
  flex: '1',
});
