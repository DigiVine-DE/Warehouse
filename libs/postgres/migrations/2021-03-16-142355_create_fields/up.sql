-- Your SQL goes here

 
  --  ______ _      _     _     
  -- |  ____(_)    | |   | |    
  -- | |__   _  ___| | __| |___ 
  -- |  __| | |/ _ \ |/ _` / __|
  -- | |    | |  __/ | (_| \__ \
  -- |_|    |_|\___|_|\__,_|___/
                             
                            
CREATE TABLE public.fields
(
	  id SERIAL PRIMARY KEY
);


 
  --  _____  _             _   _               _____             
  -- |  __ \| |           | | (_)             |  __ \            
  -- | |__) | | __ _ _ __ | |_ _ _ __   __ _  | |__) |   _ _ __  
  -- |  ___/| |/ _` | '_ \| __| | '_ \ / _` | |  _  / | | | '_ \ 
  -- | |    | | (_| | | | | |_| | | | | (_| | | | \ \ |_| | | | |
  -- |_|    |_|\__,_|_| |_|\__|_|_| |_|\__, | |_|  \_\__,_|_| |_|
  --                                    __/ |                    
  --                                   |___/                     
 


-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE public.planting_run
(
    id SERIAL NOT NULL,
    user_id Integer NOT NULL, -- ID of the user uploading
    terminal_id Integer NOT NULL, -- ID of the uploading device
    field_id Integer NOT NULL, -- ID of the wine field
    bounding_box geometry,
    CONSTRAINT planting_run_pkey PRIMARY KEY (id),
    CONSTRAINT planting_run_fk FOREIGN KEY(field_id)  
    REFERENCES public.fields(id) ON DELETE CASCADE
);



 
  --  _____  _             _   
  -- |  __ \| |           | |  
  -- | |__) | | __ _ _ __ | |_ 
  -- |  ___/| |/ _` | '_ \| __|
  -- | |    | | (_| | | | | |_ 
  -- |_|    |_|\__,_|_| |_|\__|

CREATE TABLE public.plant
(
    id SERIAL NOT NULL,
    planting_run_id Integer NOT NULL,
    import_key VARCHAR(255) NOT NULL,
    "timestamp" timestamp without time zone NOT NULL,
    height double precision NOT NULL,
    deviation double precision NOT NULL,
    speed double precision NOT NULL,
    plant_distance double precision NOT NULL, -- Maybe Integer, in mm. Awaiting more information
    row_distance double precision NOT NULL, -- Same
    row_number Integer NOT NULL,
    lat double precision NOT NULL,
    lon double precision NOT NULL,
    geo_point geometry(point) ,
    CONSTRAINT plant_pkey PRIMARY KEY (id),
    CONSTRAINT plant_fk FOREIGN KEY(planting_run_id)  
    REFERENCES public.planting_run(id) ON DELETE CASCADE
);

-- trigger to insert geo_point in plant and create bounding box for the point.
CREATE FUNCTION plant_insert_trigger_function()
RETURNS trigger AS '
BEGIN
	IF NEW.lat IS NOT NULL OR NEW.lon IS NOT NULL  THEN
		NEW.geo_point := ST_MakePoint(NEW.lon,NEW.lat);
	END IF;
  RETURN NEW;
END' LANGUAGE 'plpgsql';

CREATE TRIGGER plant_insert_trigger
BEFORE INSERT ON plant
FOR EACH ROW EXECUTE PROCEDURE plant_insert_trigger_function();

CREATE FUNCTION planting_run_insert_trigger_function()
RETURNS trigger AS '
BEGIN
   UPDATE public.planting_run wd
	SET bounding_box = sub_q.st_ext 
	FROM 
		(
		SELECT ST_Extent(geo_point) AS st_ext, wp.planting_run_id 
		FROM public.plant AS wp
		GROUP BY wp.planting_run_id
		) AS sub_q
	WHERE sub_q.planting_run_id = wd.id;
  RETURN NEW;
END' LANGUAGE 'plpgsql';

CREATE TRIGGER planting_run_insert_trigger
AFTER INSERT ON plant
FOR EACH ROW EXECUTE PROCEDURE planting_run_insert_trigger_function();