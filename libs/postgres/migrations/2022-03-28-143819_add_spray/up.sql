-- Your SQL goes here


 
CREATE EXTENSION IF NOT EXISTS postgis;

 
  --   _____                    _______        _    
  --  / ____|                  |__   __|      | |   
  -- | (___  _ __  _ __ __ _ _   _| | __ _ ___| | __
  --  \___ \| '_ \| '__/ _` | | | | |/ _` / __| |/ /
  --  ____) | |_) | | | (_| | |_| | | (_| \__ \   < 
  -- |_____/| .__/|_|  \__,_|\__, |_|\__,_|___/_|\_\
  --        | |               __/ |                 
  --        |_|              |___/                  
 


CREATE TABLE public.spraytask
(
	  task_id Integer NOT NULL,
    task_name VARCHAR NOT NULL,
    customer_name VARCHAR NOT NULL,
    farm_name VARCHAR,
    fruit_group_name VARCHAR(255) NOT NULL,
    fruit_name VARCHAR(255) NOT NULL,
    plain_name VARCHAR NOT NULL,
    plain_size BigInt NOT NULL,
    geom geometry NOT NULL,
    CONSTRAINT spray_task_pkey PRIMARY KEY (task_id)
);

CREATE VIEW jsongeo_spraytask AS
SELECT task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size,
ST_AsGeoJSON(geom,9,0)::json as json_geom  FROM spraytask;

CREATE RULE insert_jsongeo_spraytask AS ON INSERT TO jsongeo_spraytask DO INSTEAD
INSERT INTO public.spraytask(task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, geom)
VALUES (NEW.task_id, NEW.task_name, NEW.customer_name, NEW.farm_name, NEW.fruit_group_name, NEW.fruit_name, NEW.plain_name, NEW.plain_size, ST_GeomFromGeoJSON(NEW.json_geom))
RETURNING  task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, ST_AsGeoJSON(geom,9,0)::json as json_geom ;

 
 
  --   _____                       
  --  / ____|                      
  -- | (___  _ __  _ __ __ _ _   _ 
  --  \___ \| '_ \| '__/ _` | | | |
  --  ____) | |_) | | | (_| | |_| |
  -- |_____/| .__/|_|  \__,_|\__, |
  --        | |               __/ |
  --        |_|              |___/ 
 


 CREATE TABLE public.spraycomplete
(
	  task_id Integer NOT NULL,
    task_name VARCHAR NOT NULL,
    customer_name VARCHAR NOT NULL,
    farm_name VARCHAR,
    fruit_group_name VARCHAR(255) NOT NULL,
    fruit_name VARCHAR(255) NOT NULL,
    plain_name VARCHAR NOT NULL,
    plain_size BigInt NOT NULL,
    application_total_volume BigInt,
    total_area BigInt,
    effective_total_distance BigInt,
    effective_total_time BigInt, 
    geom geometry NOT NULL,
    CONSTRAINT spray_complete_pkey PRIMARY KEY (task_id)
);  

CREATE VIEW jsongeo_spraycomplete AS
SELECT task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, application_total_volume, total_area, effective_total_distance, effective_total_time,
ST_AsGeoJSON(geom,9,0)::json as json_geom  FROM spraycomplete;

CREATE RULE insert_jsongeo_spraycomplete AS ON INSERT TO jsongeo_spraycomplete DO INSTEAD
INSERT INTO public.spraycomplete(task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, application_total_volume, total_area, effective_total_distance, effective_total_time, geom)
VALUES (NEW.task_id, NEW.task_name, NEW.customer_name, NEW.farm_name, NEW.fruit_group_name, NEW.fruit_name, NEW.plain_name, NEW.plain_size, NEW.application_total_volume, NEW.total_area, NEW.effective_total_distance, NEW.effective_total_time, ST_GeomFromGeoJSON(NEW.json_geom))
RETURNING  task_id, task_name, customer_name, farm_name, fruit_group_name, fruit_name, plain_name, plain_size, application_total_volume, total_area, effective_total_distance, effective_total_time, ST_AsGeoJSON(geom,9,0)::json as json_geom ;
 

 
 
  --  _____  _             _   _____           _            _   _             
  -- |  __ \| |           | | |  __ \         | |          | | (_)            
  -- | |__) | | __ _ _ __ | |_| |__) | __ ___ | |_ ___  ___| |_ _  ___  _ __  
  -- |  ___/| |/ _` | '_ \| __|  ___/ '__/ _ \| __/ _ \/ __| __| |/ _ \| '_ \ 
  -- | |    | | (_| | | | | |_| |   | | | (_) | ||  __/ (__| |_| | (_) | | | |
  -- |_|    |_|\__,_|_| |_|\__|_|   |_|  \___/ \__\___|\___|\__|_|\___/|_| |_|
                                                                           
                                                                           
                                                                 
 CREATE TABLE public.spray_plantprotection
(
    id SERIAL NOT NULL,
	  task_id Integer,
	  complete_id Integer,
    name VARCHAR NOT NULL,
    amount BigInt NOT NULL,
    unit VARCHAR NOT NULL,

    CONSTRAINT spray_plantprotection_pkey PRIMARY KEY (id),

    CONSTRAINT spray_plantprotection_task_fk FOREIGN KEY(task_id)  
    REFERENCES public.spraytask(task_id) ON DELETE CASCADE,

    CONSTRAINT spray_plantprotection_complete_fk FOREIGN KEY(complete_id)  
    REFERENCES public.spraycomplete(task_id) ON DELETE CASCADE
);    