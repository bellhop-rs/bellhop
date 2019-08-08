ALTER TABLE leases DROP CONSTRAINT times_are_finite;

UPDATE leases SET end_time = '294277-01-09 04:00:54.77Z' WHERE end_time IS NULL;
ALTER TABLE leases ALTER COLUMN end_time SET NOT NULL;
