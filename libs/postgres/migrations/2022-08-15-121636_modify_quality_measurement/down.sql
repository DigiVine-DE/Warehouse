-- This file should undo anything in `up.sql`
CREATE EXTENSION IF NOT EXISTS postgis;

alter table public.quality_measurement
  add lat double precision;

alter table public.quality_measurement
  add lon double precision;
  
update public.quality_measurement set lon = ST_X(geo_point);
update public.quality_measurement set lat = ST_Y(geo_point);

ALTER TABLE public.quality_measurement ALTER COLUMN lon SET NOT NULL;
ALTER TABLE public.quality_measurement ALTER COLUMN lat SET NOT NULL;

-- we first drop the column to make sure that the order is correct.
alter table public.quality_measurement
  drop column geo_point;

alter table public.quality_measurement
  add geo_point geometry(point);

update public.quality_measurement set geo_point = ST_Point(lon,lat);

-- trigger to insert quality_measurement in plant and create bounding box for the point.
CREATE FUNCTION quality_measurement_insert_trigger_function()
RETURNS trigger AS '
BEGIN
	IF NEW.lat IS NOT NULL OR NEW.lon IS NOT NULL  THEN
		NEW.geo_point := ST_MakePoint(NEW.lon,NEW.lat);
	END IF;
  RETURN NEW;
END' LANGUAGE 'plpgsql';

CREATE TRIGGER quality_measurement_insert_trigger
BEFORE INSERT ON quality_measurement
FOR EACH ROW EXECUTE PROCEDURE quality_measurement_insert_trigger_function();
