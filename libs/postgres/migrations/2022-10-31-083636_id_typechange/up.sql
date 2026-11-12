-- Field
ALTER TABLE fields ALTER COLUMN id TYPE BIGINT;
ALTER TABLE fields ALTER COLUMN station_id TYPE BIGINT;

-- planting_run
ALTER TABLE planting_run ALTER COLUMN id TYPE BIGINT;
ALTER TABLE planting_run ALTER COLUMN user_id TYPE BIGINT;
ALTER TABLE planting_run ALTER COLUMN terminal_id TYPE BIGINT;
ALTER TABLE planting_run ALTER COLUMN field_id TYPE BIGINT;

-- plant
ALTER TABLE plant ALTER COLUMN id TYPE BIGINT;
ALTER TABLE plant ALTER COLUMN planting_run_id TYPE BIGINT;

-- forecast
ALTER TABLE forecast ALTER COLUMN id TYPE BIGINT;
ALTER TABLE forecast ALTER COLUMN group_id TYPE BIGINT;
ALTER TABLE forecast ALTER COLUMN station_id TYPE BIGINT;
ALTER TABLE forecast ALTER COLUMN field_ids TYPE BIGINT ARRAY;
ALTER TABLE forecast ALTER COLUMN model_id TYPE BIGINT;

-- forecast measurement
ALTER TABLE forecast_measurement ALTER COLUMN id TYPE BIGINT;
ALTER TABLE forecast_measurement ALTER COLUMN forecast_id TYPE BIGINT;

-- Harvest task
ALTER TABLE harvesttask ALTER COLUMN task_id TYPE BIGINT;

-- Harvest complete
ALTER TABLE harvestcomplete ALTER COLUMN task_id TYPE BIGINT;

-- Harvest measurement
ALTER TABLE harvestmeasurement ALTER COLUMN id TYPE BIGINT;
ALTER TABLE harvestmeasurement ALTER COLUMN task_id TYPE BIGINT;

-- quality_measurement
ALTER TABLE quality_measurement ALTER COLUMN id TYPE BIGINT;
ALTER TABLE quality_measurement ALTER COLUMN plant_id TYPE BIGINT;

-- spraytask
ALTER TABLE spraytask ALTER COLUMN task_id TYPE BIGINT;

-- spraycomplete
ALTER TABLE spraycomplete ALTER COLUMN task_id TYPE BIGINT;

-- spray_plantprotection
ALTER TABLE spray_plantprotection ALTER COLUMN id TYPE BIGINT;
ALTER TABLE spray_plantprotection ALTER COLUMN task_id TYPE BIGINT;
ALTER TABLE spray_plantprotection ALTER COLUMN complete_id TYPE BIGINT;

-- Serials          
ALTER SEQUENCE fields_id_seq AS BIGINT;
ALTER SEQUENCE forecast_id_seq AS BIGINT;
ALTER SEQUENCE forecast_measurement_id_seq AS BIGINT;
ALTER SEQUENCE harvestmeasurement_id_seq AS BIGINT;
ALTER SEQUENCE plant_id_seq AS BIGINT;
ALTER SEQUENCE planting_run_id_seq AS BIGINT;
ALTER SEQUENCE quality_measurement_id_seq AS BIGINT;
ALTER SEQUENCE spray_plantprotection_id_seq AS BIGINT;              
