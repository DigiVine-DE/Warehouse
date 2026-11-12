-- This file should undo anything in `up.sql`

DROP RULE insert_jsongeo_spraytask ON jsongeo_spraytask;
DROP VIEW jsongeo_spraytask;

DROP RULE insert_jsongeo_spraycomplete ON jsongeo_spraycomplete;
DROP VIEW jsongeo_spraycomplete;

DROP TABLE public.spraymeasurement;
DROP TABLE public.spraycomplete;
DROP TABLE public.spraytask;