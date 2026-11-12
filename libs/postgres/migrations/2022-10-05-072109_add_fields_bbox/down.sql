-- This file should undo anything in `up.sql`
DROP TRIGGER  IF EXISTS fields_insert_or_update_trigger ON fields CASCADE;
DROP FUNCTION IF EXISTS fields_insert_or_update_trigger_function CASCADE;

alter table public.fields drop column bounding_box;

