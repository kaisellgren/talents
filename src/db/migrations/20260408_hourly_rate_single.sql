ALTER TABLE talents
  DROP COLUMN hourly_rate_min,
  DROP COLUMN hourly_rate_max,
  ADD COLUMN hourly_rate INT NOT NULL DEFAULT 0;

ALTER TABLE talents ALTER COLUMN hourly_rate DROP DEFAULT;
