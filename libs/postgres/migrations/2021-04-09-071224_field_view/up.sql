-- Your SQL goes here
CREATE VIEW jsongeo_fields AS
SELECT id, name, station_id, field_number, field_size, fruit_name, ST_AsGeoJSON(geom,9,1)::json as json_geom  FROM fields;

CREATE RULE insert_jsongeo AS ON INSERT TO jsongeo_fields DO INSTEAD
INSERT INTO public.fields(name, station_id, field_number, field_size, fruit_name, geom)
VALUES (NEW.name, NEW.station_id, NEW.field_number, NEW.field_size, NEW.fruit_name, ST_GeomFromGeoJSON(NEW.json_geom))
RETURNING  id, name, station_id, field_number, field_size, fruit_name, ST_AsGeoJSON(geom,9,1)::json as json_geom ;