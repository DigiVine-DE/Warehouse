-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS postgis;

CREATE DOMAIN geo_bbox AS geometry
  CONSTRAINT valid_geo_type CHECK (
      (ST_GeometryType(VALUE) = 'ST_Polygon' AND ST_NRings(VALUE) = 1)
      OR (ST_GeometryType(VALUE) = 'ST_LineString' AND ST_NPoints(VALUE) = 2)
      OR ST_GeometryType(VALUE) = 'ST_Point'
  )
  CONSTRAINT correct_bounding_values CHECK (ST_Envelope(VALUE) = VALUE);
