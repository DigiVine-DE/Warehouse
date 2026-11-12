-- This file should undo anything in `up.sql`

ALTER TABLE public.fields 
DROP COLUMN field_number,
DROP COLUMN field_size,
DROP COLUMN fruit_name,
DROP COLUMN geom;