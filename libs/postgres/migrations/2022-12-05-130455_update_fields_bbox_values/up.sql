ALTER TABLE public.fields
ALTER COLUMN bounding_box
SET DEFAULT Box2d(ST_Point(0, 0));

UPDATE public.fields SET bounding_box = COALESCE(ST_Envelope(geom), (SELECT ST_Extent(bounding_box) FROM planting_run WHERE field_id = fields.id), Box2d(ST_Point(0, 0)));

-- Default value of BBox is [0 0, 0 0] but no more NULL
ALTER TABLE public.fields ALTER COLUMN bounding_box SET NOT NULL;

CREATE OR REPLACE FUNCTION fields_insert_or_update_trigger_function()
RETURNS trigger AS '
BEGIN
  update public.fields set bounding_box = COALESCE(ST_Envelope(geom), (SELECT ST_Extent(bounding_box) FROM planting_run WHERE field_id = NEW.id), Box2d(ST_Point(0, 0))) where id = NEW.id;
  RETURN NEW;
END' LANGUAGE 'plpgsql';

DROP TRIGGER IF EXISTS fields_insert_or_update_trigger ON public.fields;

CREATE TRIGGER fields_insert_or_update_trigger
AFTER INSERT OR UPDATE OF geom ON fields
FOR EACH ROW EXECUTE PROCEDURE fields_insert_or_update_trigger_function();

CREATE OR REPLACE FUNCTION planting_run_modify_trigger_function()
RETURNS trigger AS '
BEGIN
  IF texteq(tg_op, ''DELETE'') THEN 
   update public.fields set bounding_box = COALESCE(ST_Envelope(geom), (SELECT ST_Extent(bounding_box) FROM planting_run WHERE field_id = OLD.field_id), Box2d(ST_Point(0, 0))) where id = OLD.field_id;
  ELSE 
   update public.fields set bounding_box = COALESCE(ST_Envelope(geom), (SELECT ST_Extent(bounding_box) FROM planting_run WHERE field_id = NEW.field_id), Box2d(ST_Point(0, 0))) where id = NEW.field_id;
  END IF;
  RETURN NEW;
END' LANGUAGE 'plpgsql';

CREATE TRIGGER planting_run_modify_trigger
AFTER INSERT OR DELETE OR UPDATE OF bounding_box ON planting_run
FOR EACH ROW EXECUTE PROCEDURE planting_run_modify_trigger_function();




