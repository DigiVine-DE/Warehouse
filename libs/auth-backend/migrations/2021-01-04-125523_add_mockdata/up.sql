-- Your SQL goes here

INSERT INTO public.token VALUES (1, 'all', 5);
INSERT INTO public.token VALUES (2, 'all_readonly', 5);
INSERT INTO public.token VALUES (3, 'wagner_read', 5);
INSERT INTO public.token VALUES (4, 'none', 5);
ALTER SEQUENCE token_id_seq RESTART WITH 5;

INSERT INTO public.accessrights VALUES (1, '*', FALSE);
INSERT INTO public.accessrights VALUES (2, '*', TRUE);
INSERT INTO public.accessrights VALUES (3, '/wagner%', TRUE);