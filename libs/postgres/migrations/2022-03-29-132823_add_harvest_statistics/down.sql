-- This file should undo anything in `up.sql`
ALTER TABLE public.harvestcomplete
  DROP COLUMN "average_tartaric_acid";
ALTER TABLE public.harvestcomplete
  DROP COLUMN "average_malic_acid";
ALTER TABLE public.harvestcomplete
  DROP COLUMN "average_glucose";
ALTER TABLE public.harvestcomplete
  DROP COLUMN "average_fructose";

-- Restore VIEW and rules
DROP RULE insert_jsongeo_harvestcomplete ON jsongeo_harvestcomplete;
DROP VIEW jsongeo_harvestcomplete;

CREATE OR REPLACE VIEW jsongeo_harvestcomplete AS
SELECT task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, yield_total_mass, total_area, effective_total_distance, effective_total_time,
ST_AsGeoJSON(geom,9,0)::json as json_geom  FROM harvestcomplete;

CREATE OR REPLACE RULE insert_jsongeo_harvestcomplete AS ON INSERT TO jsongeo_harvestcomplete DO INSTEAD
INSERT INTO public.harvestcomplete(task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, yield_total_mass, total_area, effective_total_distance, effective_total_time, geom)
VALUES (NEW.task_id, NEW.task_name, NEW.customer_name, NEW.farm_name, NEW.fruit_group_name, NEW.fruit_name, NEW.plain_name, NEW.plain_size, NEW.yield_total_mass, NEW.total_area, NEW.effective_total_distance, NEW.effective_total_time, ST_GeomFromGeoJSON(NEW.json_geom))
RETURNING  task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, yield_total_mass, total_area, effective_total_distance, effective_total_time, ST_AsGeoJSON(geom,9,0)::json as json_geom ;