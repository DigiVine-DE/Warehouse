CREATE EXTENSION IF NOT EXISTS postgis;

UPDATE public.harvestmeasurement
SET geom = null
WHERE ST_IsEmpty(geom);

UPDATE public.harvestcomplete
SET geom = null
WHERE ST_IsEmpty(geom);
  
UPDATE public.fields
SET geom = null
WHERE ST_IsEmpty(geom);

DELETE FROM public.quality_measurement WHERE ST_IsEmpty(geo_point);
DELETE FROM public.spraytask WHERE ST_IsEmpty(geom);
DELETE FROM public.spraycomplete WHERE ST_IsEmpty(geom);
DELETE FROM public.harvesttask WHERE ST_IsEmpty(geom);
DELETE FROM public.plant WHERE ST_IsEmpty(geo_point);
