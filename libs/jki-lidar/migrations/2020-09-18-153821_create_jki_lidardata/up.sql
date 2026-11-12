-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE public.jkilidardata
(
    jki_lidar_id SERIAL NOT NULL,
    x double precision NOT NULL,
    y double precision NOT NULL,
    z double precision NOT NULL,
    gpstime double precision,
    intensity integer,
    return_number integer,
    number_of_returns integer,
    scan_direction_flag integer,
    edge_of_flight_line integer,
    classification integer,
    synthetic boolean,
    keypoint boolean,
    withheld boolean,
    scan_angle_rank integer,
    user_data integer,
    point_source_id integer,
    CONSTRAINT jkilidardata_pkey PRIMARY KEY (jki_lidar_id)
);
