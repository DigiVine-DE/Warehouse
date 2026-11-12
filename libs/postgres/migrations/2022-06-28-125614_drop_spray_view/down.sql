-- This file should undo anything in `up.sql`

CREATE VIEW jsongeo_spraytask AS
SELECT task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size,
ST_AsGeoJSON(geom,9,0)::json as json_geom  FROM spraytask;

CREATE RULE insert_jsongeo_spraytask AS ON INSERT TO jsongeo_spraytask DO INSTEAD
INSERT INTO public.spraytask(task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, geom)
VALUES (NEW.task_id, NEW.task_name, NEW.customer_name, NEW.farm_name, NEW.fruit_group_name, NEW.fruit_name, NEW.plain_name, NEW.plain_size, ST_GeomFromGeoJSON(NEW.json_geom))
RETURNING  task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, ST_AsGeoJSON(geom,9,0)::json as json_geom ;

CREATE VIEW jsongeo_spraycomplete AS
SELECT task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, application_total_volume, total_area, effective_total_distance, effective_total_time,
ST_AsGeoJSON(geom,9,0)::json as json_geom  FROM spraycomplete;

CREATE RULE insert_jsongeo_spraycomplete AS ON INSERT TO jsongeo_spraycomplete DO INSTEAD
INSERT INTO public.spraycomplete(task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, application_total_volume, total_area, effective_total_distance, effective_total_time, geom)
VALUES (NEW.task_id, NEW.task_name, NEW.customer_name, NEW.farm_name, NEW.fruit_group_name, NEW.fruit_name, NEW.plain_name, NEW.plain_size, NEW.application_total_volume, NEW.total_area, NEW.effective_total_distance, NEW.effective_total_time, ST_GeomFromGeoJSON(NEW.json_geom))
RETURNING  task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, application_total_volume, total_area, effective_total_distance, effective_total_time, ST_AsGeoJSON(geom,9,0)::json as json_geom ;
 
