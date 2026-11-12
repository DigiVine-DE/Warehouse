-- This file should undo anything in `up.sql`
DROP RULE insert_jsongeo_harvesttask ON jsongeo_harvesttask;
DROP VIEW jsongeo_harvesttask;

DROP RULE insert_jsongeo_harvestcomplete ON jsongeo_harvestcomplete;
DROP VIEW jsongeo_harvestcomplete;

DROP RULE insert_jsongeo_harvestmeasurement ON jsongeo_harvestcomplete;
DROP VIEW jsongeo_harvestmeasurement;

DROP TABLE public.harvestmeasurement;
DROP TABLE public.harvestcomplete;
DROP TABLE public.harvesttask;