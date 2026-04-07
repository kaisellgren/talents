export interface Talent {
  id: string;
  name: string;
  skills: string[];
  location_city: string;
  location_country: string;
  role: string | null;
  available: boolean;
  hourly_rate_min: number | null;
  hourly_rate_max: number | null;
  biography: string | null;
  created_at: string;
}

export interface AgentTalent {
  id: string;
  name: string;
  score: number;
  reasoning: string;
  summary: string;
  skills: string[];
  location_city: string;
  location_country: string;
  role: string | null;
  hourly_rate_min: number | null;
  hourly_rate_max: number | null;
  biography: string | null;
}

export interface AgentResponse {
  talents: AgentTalent[];
  iterations: number;
}
