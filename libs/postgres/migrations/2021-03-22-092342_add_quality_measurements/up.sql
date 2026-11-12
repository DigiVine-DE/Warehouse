CREATE TABLE public.quality_measurement
(
    id SERIAL NOT NULL,
    plant_id Integer NOT NULL,
    "timestamp" timestamp without time zone NOT NULL,
    device varchar NOT NULL,
    measurement_uuid varchar NOT NULL,
    tartaric_aid double precision NOT NULL,
    malic_acid double precision NOT NULL,
    fructose double precision NOT NULL,
    glucose double precision NOT NULL,
    lat double precision NOT NULL,
    lon double precision NOT NULL,
    geo_point geometry(point),
    CONSTRAINT quality_measurement_pkey PRIMARY KEY (id),
    CONSTRAINT quality_measurement_fk FOREIGN KEY(plant_id)  
    REFERENCES public.plant(id) ON DELETE CASCADE
);

-- trigger to insert geo_point in plant and create bounding box for the point.
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