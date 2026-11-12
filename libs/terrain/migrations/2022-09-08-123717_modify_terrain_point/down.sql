-- This file should undo anything in `up.sql`
CREATE EXTENSION IF NOT EXISTS postgis;

ALTER TABLE public.terrain_point ADD lat DOUBLE PRECISION;
ALTER TABLE public.terrain_point ADD lon DOUBLE PRECISION;
ALTER TABLE public.terrain_point ADD height DOUBLE PRECISION;

UPDATE public.terrain_point SET lon = ST_X(geo_point_z);
UPDATE public.terrain_point SET lat = ST_Y(geo_point_z);
UPDATE public.terrain_point SET height = ST_Z(geo_point_z);

ALTER TABLE public.terrain_point ALTER COLUMN lat SET NOT NULL;
ALTER TABLE public.terrain_point ALTER COLUMN lon SET NOT NULL;
ALTER TABLE public.terrain_point ALTER COLUMN height SET NOT NULL;

alter table public.terrain_point drop column geo_point_z;
