CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE public.shapedata
(
	jki_shape_id SERIAL NOT NULL, 
    id double precision NOT NULL,
    gridcode double precision  NOT NULL,
    orig_fid double precision  NOT NULL,
    geometry geometry,
    CONSTRAINT jki_shape_pkey PRIMARY KEY (jki_shape_id)
);

CREATE INDEX shapedata_geometry_idx ON public.shapedata USING gist (geometry) TABLESPACE pg_default;
	