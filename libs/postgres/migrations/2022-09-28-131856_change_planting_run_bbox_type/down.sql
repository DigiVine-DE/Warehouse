-- This file should undo anything in `up.sql`
CREATE EXTENSION IF NOT EXISTS postgis;

ALTER TABLE public.planting_run
  ALTER COLUMN bounding_box TYPE geometry;

