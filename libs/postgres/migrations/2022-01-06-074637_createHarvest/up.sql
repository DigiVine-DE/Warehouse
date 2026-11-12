-- Your SQL goes here

 
 --   _    _                           _     _______        _    
 --  | |  | |                         | |   |__   __|      | |   
 --  | |__| | __ _ _ ____   _____  ___| |_     | | __ _ ___| | __
 --  |  __  |/ _` | '__\ \ / / _ \/ __| __|    | |/ _` / __| |/ /
 --  | |  | | (_| | |   \ V /  __/\__ \ |_     | | (_| \__ \   < 
 --  |_|  |_|\__,_|_|    \_/ \___||___/\__|    |_|\__,_|___/_|\_\
CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE public.harvesttask
(
	task_id Integer NOT NULL,
    task_name VARCHAR NOT NULL,
    customer_name VARCHAR NOT NULL,
    farm_name VARCHAR,
    fruit_group_name VARCHAR(255) NOT NULL,
    fruit_name VARCHAR(255) NOT NULL,
    plain_name VARCHAR NOT NULL,
    plain_size BigInt NOT NULL,
    yield_total_mass BigInt,
    total_area BigInt,
    effective_total_distance BigInt,
    effective_total_time BigInt, 
    geom geometry NOT NULL,
    CONSTRAINT harvest_task_pkey PRIMARY KEY (task_id)
);

CREATE VIEW jsongeo_harvesttask AS
SELECT task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, yield_total_mass, total_area, effective_total_distance, effective_total_time,
ST_AsGeoJSON(geom,9,0)::json as json_geom  FROM harvesttask;

CREATE RULE insert_jsongeo_harvesttask AS ON INSERT TO jsongeo_harvesttask DO INSTEAD
INSERT INTO public.harvesttask(task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, yield_total_mass, total_area, effective_total_distance, effective_total_time, geom)
VALUES (NEW.task_id, NEW.task_name, NEW.customer_name, NEW.farm_name, NEW.fruit_group_name, NEW.fruit_name, NEW.plain_name, NEW.plain_size, NEW.yield_total_mass, NEW.total_area, NEW.effective_total_distance, NEW.effective_total_time, ST_GeomFromGeoJSON(NEW.json_geom))
RETURNING  task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, yield_total_mass, total_area, effective_total_distance, effective_total_time, ST_AsGeoJSON(geom,9,0)::json as json_geom ;

 
 --   _    _                           _   
 --  | |  | |                         | |  
 --  | |__| | __ _ _ ____   _____  ___| |_ 
 --  |  __  |/ _` | '__\ \ / / _ \/ __| __|
 --  | |  | | (_| | |   \ V /  __/\__ \ |_ 
 --  |_|  |_|\__,_|_|    \_/ \___||___/\__|
                                        
 CREATE TABLE public.harvestcomplete
(
	task_id Integer NOT NULL,
    task_name VARCHAR NOT NULL,
    customer_name VARCHAR NOT NULL,
    farm_name VARCHAR,
    fruit_group_name VARCHAR(255) NOT NULL,
    fruit_name VARCHAR(255) NOT NULL,
    plain_name VARCHAR NOT NULL,
    plain_size BigInt NOT NULL,
    yield_total_mass BigInt,
    total_area BigInt,
    effective_total_distance BigInt,
    effective_total_time BigInt, 
    geom geometry,
    CONSTRAINT harvest_complete_pkey PRIMARY KEY (task_id)
);  

CREATE VIEW jsongeo_harvestcomplete AS
SELECT task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, yield_total_mass, total_area, effective_total_distance, effective_total_time,
ST_AsGeoJSON(geom,9,0)::json as json_geom  FROM harvestcomplete;

CREATE RULE insert_jsongeo_harvestcomplete AS ON INSERT TO jsongeo_harvestcomplete DO INSTEAD
INSERT INTO public.harvestcomplete(task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, yield_total_mass, total_area, effective_total_distance, effective_total_time, geom)
VALUES (NEW.task_id, NEW.task_name, NEW.customer_name, NEW.farm_name, NEW.fruit_group_name, NEW.fruit_name, NEW.plain_name, NEW.plain_size, NEW.yield_total_mass, NEW.total_area, NEW.effective_total_distance, NEW.effective_total_time, ST_GeomFromGeoJSON(NEW.json_geom))
RETURNING  task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, yield_total_mass, total_area, effective_total_distance, effective_total_time, ST_AsGeoJSON(geom,9,0)::json as json_geom ;
 

 
 --   _    _                           _                            
 --  | |  | |                         | |                           
 --  | |__| | __ _ _ ____   _____  ___| |_                          
 --  |  __  |/ _` | '__\ \ / / _ \/ __| __|                         
 --  | |  | | (_| | |   \ V /  __/\__ \ |_                          
 --  |_|  |_|\__,_|_|    \_/ \___||___/\__|                     _   
 --  |  \/  |                                                  | |  
 --  | \  / | ___  __ _ ___ _   _ _ __ ___ _ __ ___   ___ _ __ | |_ 
 --  | |\/| |/ _ \/ _` / __| | | | '__/ _ \ '_ ` _ \ / _ \ '_ \| __|
 --  | |  | |  __/ (_| \__ \ |_| | | |  __/ | | | | |  __/ | | | |_ 
 --  |_|  |_|\___|\__,_|___/\__,_|_|  \___|_| |_| |_|\___|_| |_|\__|
                                                                 
                                                                 
 CREATE TABLE public.harvestmeasurement
(
    id SERIAL NOT NULL,
	task_id Integer NOT NULL,
    acid BigInt NOT NULL,
    sugar BigInt NOT NULL,
    spectral VARCHAR,
    geom geometry,

    CONSTRAINT harvest_measurement_pkey PRIMARY KEY (id),
    CONSTRAINT harvest_measurement_fk FOREIGN KEY(task_id)  
    REFERENCES public.harvestcomplete(task_id) ON DELETE CASCADE
);    

CREATE VIEW jsongeo_harvestmeasurement AS
SELECT id, task_id, acid, sugar, spectral, ST_AsGeoJSON(geom,9,0)::json as json_geom  FROM harvestmeasurement;

CREATE RULE insert_jsongeo_harvestmeasurement AS ON INSERT TO jsongeo_harvestmeasurement DO INSTEAD
INSERT INTO public.harvestmeasurement(task_id, acid, sugar, spectral, geom)
VALUES (NEW.task_id, NEW.acid, NEW.sugar, NEW.spectral, ST_GeomFromGeoJSON(NEW.json_geom))
RETURNING  id, task_id, acid, sugar, spectral, ST_AsGeoJSON(geom,9,0)::json as json_geom ;
