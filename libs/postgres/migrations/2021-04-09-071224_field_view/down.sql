-- This file should undo anything in `up.sql`
DROP RULE insert_jsongeo ON jsongeo_fields;
DROP VIEW jsongeo_fields;