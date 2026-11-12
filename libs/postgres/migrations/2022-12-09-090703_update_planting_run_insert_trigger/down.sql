-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS plant_modify_trigger ON public.plant;

CREATE TRIGGER planting_run_insert_trigger
AFTER INSERT ON plant
FOR EACH ROW EXECUTE PROCEDURE planting_run_insert_trigger_function();
