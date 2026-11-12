-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS postgis;

update public.quality_measurement set geo_point = ST_Point(lon,lat);

ALTER TABLE public.quality_measurement ALTER COLUMN geo_point SET NOT NULL;

alter table public.quality_measurement
  drop column lat;

alter table public.quality_measurement
  drop column lon;

DROP trigger  IF EXISTS quality_measurement_insert_trigger ON quality_measurement CASCADE;
DROP FUNCTION IF EXISTS quality_measurement_insert_trigger_function CASCADE;

