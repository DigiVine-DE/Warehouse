-- Your SQL goes here
UPDATE public.terrain_point SET geo_point_z = ST_SetSRID(geo_point_z, 4326);
