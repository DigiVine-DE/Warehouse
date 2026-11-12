-- Your SQL goes here
alter table public.fields add bounding_box geo_bbox;

update public.fields set bounding_box = ST_Envelope(geom);

CREATE FUNCTION fields_insert_or_update_trigger_function()
RETURNS trigger AS '
BEGIN
  update public.fields set bounding_box = ST_Envelope(geom) where id = NEW.id;
  RETURN NEW;
END' LANGUAGE 'plpgsql';

CREATE TRIGGER fields_insert_or_update_trigger
AFTER INSERT OR UPDATE OF geom ON fields
FOR EACH ROW EXECUTE PROCEDURE fields_insert_or_update_trigger_function();


