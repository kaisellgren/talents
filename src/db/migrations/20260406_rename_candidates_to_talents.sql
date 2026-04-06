-- Rename the candidates table and its indexes to talents.
ALTER TABLE candidates RENAME TO talents;

ALTER INDEX idx_candidates_available RENAME TO idx_talents_available;
ALTER INDEX idx_candidates_skills_gin RENAME TO idx_talents_skills_gin;
ALTER INDEX idx_candidates_city RENAME TO idx_talents_city;
ALTER INDEX idx_candidates_country RENAME TO idx_talents_country;
ALTER INDEX idx_candidates_avail_loc RENAME TO idx_talents_avail_loc;
