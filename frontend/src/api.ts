import type { AgentResponse, Talent } from './types';

export async function runAgent(prompt: string): Promise<AgentResponse> {
  const res = await fetch('/api/agents/run', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ prompt }),
  });
  if (!res.ok) throw new Error(`Agent run failed: ${res.status}`);
  return res.json();
}

export async function listAvailable(): Promise<Talent[]> {
  const res = await fetch('/api/talents/available');
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
  const res = await fetch(`/api/talents/search?${params}`);
  if (!res.ok) throw new Error(`Search failed: ${res.status}`);
  return res.json();
}
