-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS postgis;

update public.plant set geo_point = ST_Point(lat,lon);

ALTER TABLE public.plant ALTER COLUMN geo_point SET NOT NULL;

alter table public.plant
  drop column lat;

alter table public.plant
  drop column lon;

DROP trigger  IF EXISTS plant_insert_trigger ON plant CASCADE;
DROP FUNCTION IF EXISTS plant_insert_trigger_function CASCADE;

-- we need to update bbox values since they were calculated based on old incorrect geo_point.
UPDATE public.planting_run wd
   SET bounding_box = sub_q.st_ext 
	   FROM 
	   (
		 SELECT ST_Extent(geo_point) AS st_ext, wp.planting_run_id 
		   FROM public.plant AS wp
		  GROUP BY wp.planting_run_id
	   ) AS sub_q
 WHERE sub_q.planting_run_id = wd.id;
