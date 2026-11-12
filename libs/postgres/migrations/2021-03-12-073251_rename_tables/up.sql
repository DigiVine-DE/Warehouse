-- Your SQL goes here

ALTER TABLE vitimeteo
RENAME TO forecast;

ALTER TABLE vitimeteo_measurements
RENAME TO forecast_measurement;

ALTER TABLE forecast_measurement 
RENAME COLUMN vitimeteo_id TO forecast_id;

ALTER SEQUENCE vitimeteo_id_seq 
RENAME TO forecast_id_seq;

ALTER SEQUENCE vitimeteo_measurements_id_seq 
RENAME TO forecast_measurement_id_seq;