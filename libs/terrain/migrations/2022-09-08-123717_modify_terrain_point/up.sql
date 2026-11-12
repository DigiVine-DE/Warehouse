-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS postgis;

SELECT AddGeometryColumn ('public','terrain_point','geo_point_z',4326,'POINT',3);

UPDATE public.terrain_point SET geo_point_z = ST_PointZ(lon,lat,height, 4326);
ALTER TABLE public.terrain_point ALTER COLUMN geo_point_z SET NOT NULL;

ALTER TABLE public.terrain_point DROP COLUMN lon;
ALTER TABLE public.terrain_point DROP COLUMN lat;
ALTER TABLE public.terrain_point DROP COLUMN height;


