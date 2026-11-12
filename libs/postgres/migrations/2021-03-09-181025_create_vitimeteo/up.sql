CREATE TABLE public.vitimeteo
(
	id SERIAL PRIMARY KEY, 
    group_id INTEGER NOT NULL,
    station_id INTEGER NOT NULL,
    station VARCHAR NOT NULL,
    field_ids INTEGER ARRAY NOT NULL,
    model_id INTEGER NOT NULL,
    model VARCHAR NOT NULL,
    query_timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    time_span_start DATE NOT NULL,
    time_span_end DATE NOT NULL,
    measurement_rate VARCHAR NOT NULL
);

CREATE TABLE public.vitimeteo_measurements
(
    id SERIAL PRIMARY KEY,
    vitimeteo_id INTEGER NOT NULL REFERENCES public.vitimeteo(id),
    "time" TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    "value" JSONB NOT NULL
);
