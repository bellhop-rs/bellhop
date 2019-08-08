ALTER TABLE leases ALTER COLUMN end_time DROP NOT NULL;

-- Diesel doesn't support infinity for timestamps so we're going to use null to
-- represent infinity instead: https://github.com/diesel-rs/diesel/issues/2139
ALTER TABLE leases ADD CONSTRAINT times_are_finite CHECK (
    ISFINITE(start_time)
    AND (
        end_time IS NULL
        OR ISFINITE(end_time)
    )
);
