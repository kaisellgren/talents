import type { AgentResponse, Talent } from './types';

const API_BASE = import.meta.env.VITE_API_BASE ?? '';

export async function runAgent(prompt: string): Promise<AgentResponse> {
  const res = await fetch(`${API_BASE}/agents/run`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ prompt }),
  });
  if (!res.ok) throw new Error(`Agent run failed: ${res.status}`);
  return res.json();
}

export async function listAvailable(limit?: number, offset?: number): Promise<Talent[]> {
  const params = new URLSearchParams();
  if (limit !== undefined) params.set('limit', String(limit));
  if (offset !== undefined) params.set('offset', String(offset));
  const query = params.toString() ? `?${params}` : '';
  const res = await fetch(`${API_BASE}/talents/available${query}`);
  if (!res.ok) throw new Error(`Failed to fetch talents: ${res.status}`);
  return res.json();
}

export async function searchTalents(
  skills: string[],
  city?: string,
  country?: string,
): Promise<Talent[]> {
  const params = new URLSearchParams();
  if (skills.length > 0) params.set('skills', skills.join(','));
  if (city) params.set('city', city);
  if (country) params.set('country', country);
  const res = await fetch(`${API_BASE}/talents/search?${params}`);
  if (!res.ok) throw new Error(`Search failed: ${res.status}`);
  return res.json();
}
