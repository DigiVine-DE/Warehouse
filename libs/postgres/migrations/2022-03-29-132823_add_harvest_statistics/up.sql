-- Your SQL goes here
ALTER TABLE public.harvestcomplete
  ADD COLUMN "average_tartaric_acid" BigInt;
ALTER TABLE public.harvestcomplete
  ADD COLUMN "average_malic_acid" BigInt;
ALTER TABLE public.harvestcomplete
  ADD COLUMN "average_glucose" BigInt;
ALTER TABLE public.harvestcomplete
  ADD COLUMN "average_fructose" BigInt;

-- Update VIEW and rules
DROP RULE insert_jsongeo_harvestcomplete ON jsongeo_harvestcomplete;
DROP VIEW jsongeo_harvestcomplete;

CREATE OR REPLACE VIEW jsongeo_harvestcomplete AS
SELECT task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, yield_total_mass, total_area, effective_total_distance, effective_total_time,average_tartaric_acid, average_malic_acid, average_glucose, average_fructose, ST_AsGeoJSON(geom,9,0)::json as json_geom  FROM harvestcomplete;

CREATE OR REPLACE RULE insert_jsongeo_harvestcomplete AS ON INSERT TO jsongeo_harvestcomplete DO INSTEAD
INSERT INTO public.harvestcomplete(task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, yield_total_mass, total_area, effective_total_distance, effective_total_time,average_tartaric_acid, average_malic_acid, average_glucose, average_fructose, geom)
VALUES (NEW.task_id, NEW.task_name, NEW.customer_name, NEW.farm_name, NEW.fruit_group_name, NEW.fruit_name, NEW.plain_name, NEW.plain_size, NEW.yield_total_mass, NEW.total_area, NEW.effective_total_distance, NEW.effective_total_time, NEW.average_tartaric_acid, NEW.average_malic_acid, NEW.average_glucose, NEW.average_fructose, ST_GeomFromGeoJSON(NEW.json_geom))
RETURNING  task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, yield_total_mass, total_area, effective_total_distance, effective_total_time,average_tartaric_acid, average_malic_acid, average_glucose, average_fructose, ST_AsGeoJSON(geom,9,0)::json as json_geom ;