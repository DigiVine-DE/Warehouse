-- This file should undo anything in `up.sql`
DELETE FROM public.token;
ALTER SEQUENCE token_id_seq RESTART WITH 1;

DELETE FROM public.accessrights;