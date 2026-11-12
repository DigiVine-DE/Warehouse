-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE public.erodata
(
  ero_id SERIAL NOT NULL,
  lat double precision NOT NULL,
  lon double precision NOT NULL,
  ndvi_corrected double precision NOT NULL,
  "time" timestamp without time zone NOT NULL,
  ndvi_raw double precision NOT NULL,
  the_geom geography,
  CONSTRAINT erodata_pkey PRIMARY KEY (ero_id)
);

CREATE INDEX erodata_the_geom_idx ON public.erodata USING gist (the_geom);

-- function creates a trigger which should be executed on each insert of (lon,lat)  values. 
-- This trigger will update 'the_geom' column on the basis of (lon,lat) values at each insertion.
CREATE FUNCTION ero_insert_trigger_function()
RETURNS trigger AS '
BEGIN
  IF NEW.lat IS NOT NULL OR NEW.lon IS NOT NULL  THEN
    NEW.the_geom := ST_SetSRID(ST_MakePoint(NEW.lon, NEW.lat), 4326);
  END IF;
  RETURN NEW;
END' LANGUAGE 'plpgsql';

-- setting the trigger before insert on the table.
CREATE TRIGGER ero_insert_trigger
BEFORE INSERT ON erodata
FOR EACH ROW
EXECUTE PROCEDURE ero_insert_trigger_function();
