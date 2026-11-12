-- Your SQL goes here
ALTER TABLE public.fields 
ADD COLUMN field_number VARCHAR,
ADD COLUMN field_size double precision,
ADD COLUMN fruit_name VARCHAR,
ADD COLUMN geom geometry;
