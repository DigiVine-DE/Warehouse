--------------------
-- update columns --
--------------------

-- Note:
-- ST_Transform or UpdateGeometrySRID changes coordinates based on the old and new SRIDs,
-- which is not what we want.

UPDATE public.fields SET bounding_box = ST_SetSRID(bounding_box, 4326), geom = ST_SetSRID(geom, 4326);
UPDATE public.planting_run SET bounding_box = ST_SetSRID(bounding_box, 4326);
UPDATE public.quality_measurement SET geo_point = ST_SetSRID(geo_point, 4326);
UPDATE public.harvesttask SET geom = ST_SetSRID(geom, 4326);
UPDATE public.harvestcomplete SET geom = ST_SetSRID(geom, 4326);
UPDATE public.harvestmeasurement SET geom = ST_SetSRID(geom, 4326);
UPDATE public.plant SET geo_point = ST_SetSRID(geo_point, 4326);
UPDATE public.spraytask SET geom = ST_SetSRID(geom, 4326);
UPDATE public.spraycomplete SET geom = ST_SetSRID(geom, 4326);


---------------------
-- update triggers --
---------------------

-- fields
DROP TRIGGER IF EXISTS fields_insert_or_update_trigger ON public.fields;
CREATE OR REPLACE FUNCTION fields_insert_or_update_trigger_function()
RETURNS trigger AS '
BEGIN
  update public.fields set bounding_box = ST_SetSRID(COALESCE(ST_Envelope(geom), (SELECT ST_Extent(bounding_box) FROM planting_run WHERE field_id = NEW.id), Box2d(ST_Point(0, 0))), 4326) where id = NEW.id;
  RETURN NEW;
END' LANGUAGE 'plpgsql';

CREATE TRIGGER fields_insert_or_update_trigger
  AFTER INSERT OR UPDATE OF geom ON fields
  FOR EACH ROW EXECUTE PROCEDURE fields_insert_or_update_trigger_function();


-- planting_run
DROP TRIGGER IF EXISTS planting_run_modify_trigger ON public.planting_run;
DROP TRIGGER IF EXISTS plant_modify_trigger ON public.plant;

CREATE OR REPLACE FUNCTION planting_run_modify_trigger_function()
RETURNS trigger AS '
BEGIN
  IF texteq(tg_op, ''DELETE'') THEN 
   update public.fields set bounding_box = ST_SetSRID(COALESCE(ST_Envelope(geom), (SELECT ST_Extent(bounding_box) FROM planting_run WHERE field_id = OLD.field_id), Box2d(ST_Point(0, 0))), 4326) where id = OLD.field_id;
  ELSE 
   update public.fields set bounding_box = ST_SetSRID(COALESCE(ST_Envelope(geom), (SELECT ST_Extent(bounding_box) FROM planting_run WHERE field_id = NEW.field_id), Box2d(ST_Point(0, 0))), 4326) where id = NEW.field_id;
  END IF;
  RETURN NEW;
END' LANGUAGE 'plpgsql';

CREATE TRIGGER planting_run_modify_trigger
AFTER INSERT OR DELETE OR UPDATE OF bounding_box ON planting_run
FOR EACH ROW EXECUTE PROCEDURE planting_run_modify_trigger_function();

CREATE TRIGGER plant_modify_trigger
AFTER INSERT OR UPDATE OR DELETE ON plant
FOR EACH ROW EXECUTE PROCEDURE planting_run_insert_trigger_function();




