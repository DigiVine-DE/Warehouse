-- This file should undo anything in `up.sql`
ALTER TABLE quality_measurement
RENAME COLUMN tartaric_acid  TO tartaric_aid;