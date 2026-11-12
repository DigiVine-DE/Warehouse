DELETE FROM public.terrain_point WHERE ST_IsEmpty(geo_point_z);
