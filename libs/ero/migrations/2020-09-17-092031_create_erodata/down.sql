-- This file should undo anything in `up.sql`

DROP TRIGGER IF EXISTS ero_insert_trigger ON public.erodata;

DROP FUNCTION if exists ero_insert_trigger_function CASCADE;

DROP INDEX erodata_the_geom_idx;

DROP TABLE erodata;

