-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS postgis;

ALTER TABLE public.planting_run
  ALTER COLUMN bounding_box TYPE geo_bbox;

