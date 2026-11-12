-- This file should undo anything in `up.sql`

ALTER TABLE public.fields ALTER COLUMN bounding_box DROP NOT NULL;
ALTER TABLE public.fields ALTER COLUMN bounding_box DROP DEFAULT;
UPDATE public.fields SET bounding_box = ST_Envelope(geom);


CREATE OR REPLACE FUNCTION fields_insert_or_update_trigger_function()
RETURNS trigger AS '
BEGIN
  update public.fields set bounding_box = ST_Envelope(geom) where id = NEW.id;
  RETURN NEW;
END' LANGUAGE 'plpgsql';

DROP TRIGGER IF EXISTS fields_insert_or_update_trigger ON public.fields;

CREATE TRIGGER fields_insert_or_update_trigger
AFTER INSERT OR UPDATE OF geom ON fields
FOR EACH ROW EXECUTE PROCEDURE fields_insert_or_update_trigger_function();

DROP TRIGGER IF EXISTS planting_run_modify_trigger ON public.planting_run;
DROP FUNCTION planting_run_modify_trigger_function;
