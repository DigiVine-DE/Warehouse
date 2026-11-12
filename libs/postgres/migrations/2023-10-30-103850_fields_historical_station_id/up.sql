ALTER TABLE fields
ADD COLUMN historical_station_id INTEGER,
ADD CONSTRAINT fk_historical_station
FOREIGN KEY (historical_station_id)
REFERENCES weather_station (station_id);
