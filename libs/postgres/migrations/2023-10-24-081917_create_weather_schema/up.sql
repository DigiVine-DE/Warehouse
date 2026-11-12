CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE weather_station (
  station_id INTEGER PRIMARY KEY,
  station_name TEXT,
  station_bundesland TEXT
);

SELECT AddGeometryColumn('public','weather_station','station_geo_point_z',4326,'POINT',3);

CREATE TABLE weather_data (
  station_id INTEGER NOT NULL REFERENCES weather_station (station_id) ON DELETE CASCADE,
  measurement_date DATE NOT NULL,
  fx DOUBLE PRECISION,
  fx_qn SMALLINT,
  fm DOUBLE PRECISION,
  fm_qn SMALLINT,
  rsk DOUBLE PRECISION,
  rsk_qn SMALLINT,
  rskf INTEGER,
  sdk DOUBLE PRECISION,
  sdk_qn SMALLINT,
  shk_tag DOUBLE PRECISION,
  shk_tag_qn SMALLINT,
  nm DOUBLE PRECISION,
  nm_qn SMALLINT,
  vpm DOUBLE PRECISION,
  vpm_qn SMALLINT,
  pm DOUBLE PRECISION,
  pm_qn SMALLINT,
  tmk DOUBLE PRECISION,
  tmk_qn SMALLINT,
  upm DOUBLE PRECISION,
  upm_qn SMALLINT,
  txk DOUBLE PRECISION,
  txk_qn SMALLINT,
  tnk DOUBLE PRECISION,
  tnk_qn SMALLINT,
  tgk DOUBLE PRECISION,
  tgk_qn SMALLINT,
  PRIMARY KEY (station_id, measurement_date)
);

COMMENT ON COLUMN weather_data.measurement_date IS 'Reference date';
COMMENT ON COLUMN weather_data.fx IS 'Daily maximum of windgust (m/s)';
COMMENT ON COLUMN weather_data.fx_qn IS 'Quality level of daily maximum of windgust';
COMMENT ON COLUMN weather_data.fm IS 'Daily mean of wind velocity (m/s)';
COMMENT ON COLUMN weather_data.fm_qn IS 'Quality level of daily mean of wind velocity';
COMMENT ON COLUMN weather_data.rsk IS 'Daily precipitation height (mm)';
COMMENT ON COLUMN weather_data.rsk_qn IS 'Quality level of daily precipitation height';
COMMENT ON COLUMN weather_data.rskf IS 'Precipitation form (numerical code)';
COMMENT ON COLUMN weather_data.sdk IS 'Daily sunshine duration (h)';
COMMENT ON COLUMN weather_data.sdk_qn IS 'Quality level of daily sunshine duration';
COMMENT ON COLUMN weather_data.shk_tag IS 'Daily snow depth (cm)';
COMMENT ON COLUMN weather_data.shk_tag_qn IS 'Quality level of daily snow depth';
COMMENT ON COLUMN weather_data.nm IS 'Daily mean of cloud cover (1/8)';
COMMENT ON COLUMN weather_data.nm_qn IS 'Quality level of daily mean of cloud cover';
COMMENT ON COLUMN weather_data.vpm IS 'Daily mean of vapor pressure (hPa)';
COMMENT ON COLUMN weather_data.vpm_qn IS 'Quality level of daily mean of vapor pressure';
COMMENT ON COLUMN weather_data.pm IS 'Daily mean of pressure (hPa)';
COMMENT ON COLUMN weather_data.pm_qn IS 'Quality level of daily mean of pressure';
COMMENT ON COLUMN weather_data.tmk IS 'Daily mean of temperature (°C)';
COMMENT ON COLUMN weather_data.tmk_qn IS 'Quality level of daily mean of temperature';
COMMENT ON COLUMN weather_data.upm IS 'Daily mean of relative humidity (%)';
COMMENT ON COLUMN weather_data.upm_qn IS 'Quality level of daily mean of relative humidity';
COMMENT ON COLUMN weather_data.txk IS 'Daily maximum of temperature at 2 m height (°C)';
COMMENT ON COLUMN weather_data.txk_qn IS 'Quality level of daily maximum of temperature at 2 m height';
COMMENT ON COLUMN weather_data.tnk IS 'Daily minimum of temperature at 2 m height (°C)';
COMMENT ON COLUMN weather_data.tnk_qn IS 'Quality level of daily minimum of temperature at 2 m height';
COMMENT ON COLUMN weather_data.tgk IS 'Daily minimum of air temperature at 5 cm above ground (°C)';
COMMENT ON COLUMN weather_data.tgk_qn IS 'Quality level of daily minimum of air temperature at 5 cm above ground';

