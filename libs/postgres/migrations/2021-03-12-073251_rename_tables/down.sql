-- This file should undo anything in `up.sql`

ALTER TABLE forecast
RENAME TO vitimeteo;

ALTER TABLE forecast_measurement
RENAME TO vitimeteo_measurements;

ALTER TABLE forecast_measurement 
RENAME COLUMN forecast_id TO vitimeteo_id;