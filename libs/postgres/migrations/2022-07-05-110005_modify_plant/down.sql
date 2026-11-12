-- This file should undo anything in `up.sql`
CREATE EXTENSION IF NOT EXISTS postgis;

alter table public.plant
  add lat double precision;

alter table public.plant
  add lon double precision;
  
update public.plant set lat = ST_X(geo_point);
update public.plant set lon = ST_Y(geo_point);

ALTER TABLE public.plant ALTER COLUMN lat SET NOT NULL;
ALTER TABLE public.plant ALTER COLUMN lon SET NOT NULL;

-- we first drop the column to make sure that the order is correct.
alter table public.plant
  drop column geo_point;

alter table public.plant
  add geo_point geometry(point);

update public.plant set geo_point = ST_Point(lat,lon);

-- trigger to insert geo_point in plant and create bounding box for the point.
CREATE FUNCTION plant_insert_trigger_function()
RETURNS trigger AS '
BEGIN
	IF NEW.lat IS NOT NULL OR NEW.lon IS NOT NULL  THEN
		NEW.geo_point := ST_MakePoint(NEW.lon,NEW.lat);
	END IF;
  RETURN NEW;
END' LANGUAGE 'plpgsql';

CREATE TRIGGER plant_insert_trigger
BEFORE INSERT ON plant
FOR EACH ROW EXECUTE PROCEDURE plant_insert_trigger_function();

