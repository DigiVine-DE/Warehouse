-- Field
ALTER TABLE fields ALTER COLUMN id TYPE INTEGER;
ALTER TABLE fields ALTER COLUMN station_id TYPE INTEGER;

-- planting_run
ALTER TABLE planting_run ALTER COLUMN id TYPE INTEGER;
ALTER TABLE planting_run ALTER COLUMN user_id TYPE INTEGER;
ALTER TABLE planting_run ALTER COLUMN terminal_id TYPE INTEGER;
ALTER TABLE planting_run ALTER COLUMN field_id TYPE INTEGER;

-- plant
ALTER TABLE plant ALTER COLUMN id TYPE INTEGER;
ALTER TABLE plant ALTER COLUMN planting_run_id TYPE INTEGER;

-- forecast
ALTER TABLE forecast ALTER COLUMN id TYPE INTEGER;
ALTER TABLE forecast ALTER COLUMN group_id TYPE INTEGER;
ALTER TABLE forecast ALTER COLUMN station_id TYPE INTEGER;
ALTER TABLE forecast ALTER COLUMN field_ids TYPE INTEGER ARRAY;
ALTER TABLE forecast ALTER COLUMN model_id TYPE INTEGER;

-- forecast measurement
ALTER TABLE forecast_measurement ALTER COLUMN id TYPE INTEGER;
ALTER TABLE forecast_measurement ALTER COLUMN forecast_id TYPE INTEGER;

-- Harvest task
ALTER TABLE harvesttask ALTER COLUMN task_id TYPE INTEGER;

-- Harvest complete
ALTER TABLE harvestcomplete ALTER COLUMN task_id TYPE INTEGER;

-- Harvest measurement
ALTER TABLE harvestmeasurement ALTER COLUMN id TYPE INTEGER;
ALTER TABLE harvestmeasurement ALTER COLUMN task_id TYPE INTEGER;

-- quality_measurement
ALTER TABLE quality_measurement ALTER COLUMN id TYPE INTEGER;
ALTER TABLE quality_measurement ALTER COLUMN plant_id TYPE INTEGER;

-- spraytask
ALTER TABLE spraytask ALTER COLUMN task_id TYPE INTEGER;

-- spraycomplete
ALTER TABLE spraycomplete ALTER COLUMN task_id TYPE INTEGER;

-- spray_plantprotection
ALTER TABLE spray_plantprotection ALTER COLUMN id TYPE INTEGER;
ALTER TABLE spray_plantprotection ALTER COLUMN task_id TYPE INTEGER;
ALTER TABLE spray_plantprotection ALTER COLUMN complete_id TYPE INTEGER;

-- Serials          
ALTER SEQUENCE fields_id_seq AS INTEGER;
ALTER SEQUENCE forecast_id_seq AS INTEGER;
ALTER SEQUENCE forecast_measurement_id_seq AS INTEGER;
ALTER SEQUENCE harvestmeasurement_id_seq AS INTEGER;
ALTER SEQUENCE plant_id_seq AS INTEGER;
ALTER SEQUENCE planting_run_id_seq AS INTEGER;
ALTER SEQUENCE quality_measurement_id_seq AS INTEGER;
ALTER SEQUENCE spray_plantprotection_id_seq AS INTEGER;