-- This file should undo anything in `up.sql`
DROP TABLE public.fields;

DROP trigger  IF EXISTS plant_insert_trigger ON plant CASCADE;
DROP FUNCTION IF EXISTS plant_insert_trigger_function CASCADE;


DROP trigger  IF EXISTS planting_run_insert_trigger on planting_run CASCADE;
DROP function if exists planting_run_insert_trigger_function CASCADE;

DROP TABLE public.planting_run;
DROP TABLE public.plant;
