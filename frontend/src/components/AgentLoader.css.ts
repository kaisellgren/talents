import { style, keyframes } from '@vanilla-extract/css';

const sonar = keyframes({
  '0%': { transform: 'translate(-50%, -50%) scale(0.4)', opacity: 0.7 },
  '100%': { transform: 'translate(-50%, -50%) scale(2.8)', opacity: 0 },
});

const corePulse = keyframes({
  '0%, 100%': { transform: 'translate(-50%, -50%) scale(0.92)', boxShadow: '0 0 16px 4px rgba(163, 166, 255, 0.45)' },
  '50%': { transform: 'translate(-50%, -50%) scale(1.06)', boxShadow: '0 0 28px 8px rgba(163, 166, 255, 0.7)' },
});

const stepSlideIn = keyframes({
  '0%': { opacity: 0, transform: 'translateX(-6px)' },
  '100%': { opacity: 1, transform: 'translateX(0)' },
});

const dotPulse = keyframes({
  '0%, 100%': { opacity: 0.5, transform: 'scale(0.85)' },
  '50%': { opacity: 1, transform: 'scale(1.2)' },
});

const shimmer = keyframes({
  '0%': { backgroundPosition: '-400px 0' },
  '100%': { backgroundPosition: '400px 0' },
});

export const wrapper = style({
  display: 'flex',
  justifyContent: 'center',
  marginTop: '20px',
  width: '100%',
});

export const card = style({
  background: 'rgba(8, 14, 32, 0.88)',
  backdropFilter: 'blur(24px)',
  WebkitBackdropFilter: 'blur(24px)',
  border: '1px solid rgba(163, 166, 255, 0.18)',
  borderRadius: '18px',
  padding: '36px 40px',
  width: '100%',
  maxWidth: '460px',
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  gap: '28px',
  boxShadow: '0 0 60px rgba(100, 106, 255, 0.12), 0 8px 40px rgba(0, 0, 0, 0.5)',
  position: 'relative',
  overflow: 'hidden',
  '::before': {
    content: '""',
    position: 'absolute',
    inset: 0,
    borderRadius: '18px',
    background: 'linear-gradient(135deg, rgba(163,166,255,0.04) 0%, transparent 60%)',
    pointerEvents: 'none',
  },
});

export const orbWrap = style({
  position: 'relative',
  width: '80px',
  height: '80px',
  flexShrink: 0,
});

export const ring = style({
  position: 'absolute',
  top: '50%',
  left: '50%',
  width: '72px',
  height: '72px',
  borderRadius: '50%',
  border: '1.5px solid rgba(163, 166, 255, 0.5)',
  animation: `${sonar} 2.4s ease-out infinite`,
  transform: 'translate(-50%, -50%)',
});

export const ring2 = style({
  position: 'absolute',
  top: '50%',
  left: '50%',
  width: '72px',
  height: '72px',
  borderRadius: '50%',
  border: '1.5px solid rgba(163, 166, 255, 0.5)',
  animation: `${sonar} 2.4s ease-out 0.8s infinite`,
  transform: 'translate(-50%, -50%)',
});

export const ring3 = style({
  position: 'absolute',
  top: '50%',
  left: '50%',
  width: '72px',
  height: '72px',
  borderRadius: '50%',
  border: '1.5px solid rgba(163, 166, 255, 0.5)',
  animation: `${sonar} 2.4s ease-out 1.6s infinite`,
  transform: 'translate(-50%, -50%)',
});

export const core = style({
  position: 'absolute',
  top: '50%',
  left: '50%',
  width: '28px',
  height: '28px',
  borderRadius: '50%',
  background: 'radial-gradient(circle, #c5c7ff 0%, #7b7eff 55%, rgba(100,103,255,0) 100%)',
  animation: `${corePulse} 1.8s ease-in-out infinite`,
  transform: 'translate(-50%, -50%)',
});

export const label = style({
  fontFamily: "'Manrope', sans-serif",
  fontWeight: 700,
  fontSize: '1rem',
  color: '#dee5ff',
  letterSpacing: '-0.01em',
  margin: 0,
  textAlign: 'center',
});

export const sublabel = style({
  fontFamily: "'Inter', sans-serif",
  fontSize: '0.78rem',
  color: '#596482',
  margin: 0,
  textAlign: 'center',
  letterSpacing: '0.04em',
});

export const headerBlock = style({
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  gap: '6px',
});

export const steps = style({
  listStyle: 'none',
  margin: 0,
  padding: 0,
  width: '100%',
  display: 'flex',
  flexDirection: 'column',
  gap: '10px',
});

const stepBase = style({
  display: 'flex',
  alignItems: 'center',
  gap: '12px',
  fontFamily: "'Inter', sans-serif",
  fontSize: '0.875rem',
  borderRadius: '8px',
  padding: '9px 12px',
  transition: 'all 0.3s ease',
});

export const stepPending = style([stepBase, {
  color: '#2e3a55',
}]);

export const stepActive = style([stepBase, {
  color: '#dee5ff',
  background: 'rgba(163, 166, 255, 0.07)',
  border: '1px solid rgba(163, 166, 255, 0.12)',
  animation: `${stepSlideIn} 0.3s ease both`,
}]);

export const stepDone = style([stepBase, {
  color: '#4d5a78',
}]);

export const dotPending = style({
  width: '7px',
  height: '7px',
  borderRadius: '50%',
  background: '#1e2a42',
  flexShrink: 0,
});

export const dotActive = style({
  width: '7px',
  height: '7px',
  borderRadius: '50%',
  background: '#a3a6ff',
  flexShrink: 0,
  boxShadow: '0 0 8px 2px rgba(163, 166, 255, 0.6)',
  animation: `${dotPulse} 1s ease-in-out infinite`,
});

export const dotDone = style({
  width: '7px',
  height: '7px',
  borderRadius: '50%',
  background: '#3a7a5a',
  flexShrink: 0,
});

export const shimmerBar = style({
  height: '2px',
  width: '100%',
  borderRadius: '2px',
  background: 'linear-gradient(90deg, transparent 0%, rgba(163,166,255,0.15) 20%, rgba(163,166,255,0.5) 50%, rgba(163,166,255,0.15) 80%, transparent 100%)',
  backgroundSize: '800px 2px',
  animation: `${shimmer} 2s linear infinite`,
});
