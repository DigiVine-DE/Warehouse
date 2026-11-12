-- Your SQL goes here
DROP TRIGGER IF EXISTS planting_run_insert_trigger ON public.plant;

CREATE TRIGGER plant_modify_trigger
AFTER INSERT OR UPDATE OR DELETE ON plant
FOR EACH ROW EXECUTE PROCEDURE planting_run_insert_trigger_function();
