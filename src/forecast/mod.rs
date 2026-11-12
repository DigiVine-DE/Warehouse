use crate::response::{Delete, FromEntry, FromJson, Get, GetAll, Post};
use chrono::TimeZone;

use digivine_api_postgres::forecast::db::{ForecastInsert, NestedForecast, NestedForecastInsert};
use digivine_server::{
    models::{self, Forecast, ForecastMeasurement},
    DeleteOneForecastResponse, GetAllForecastsResponse, GetOneForecastResponse,
    PostForecastResponse,
};

impl GetAll<NestedForecast, models::Forecast> for GetAllForecastsResponse {}
impl Get<NestedForecast, models::Forecast> for GetOneForecastResponse {}
impl Delete<NestedForecast> for DeleteOneForecastResponse {}
impl Post<NestedForecastInsert, models::Forecast> for PostForecastResponse {}

impl FromEntry<NestedForecast> for Forecast {
    fn from_entry(val: NestedForecast) -> Result<Self, String> {
        let f = val.forecast;
        let field_ids: Vec<i64> = f.field_ids.into_iter().collect();
        Ok(Forecast {
            id: Some(f.id),
            group_id: f.group_id,
            station_id: f.station_id,
            station: f.station,
            field_ids,
            model_id: f.model_id,
            model: f.model,
            query_timestamp: chrono::Utc.from_utc_datetime(&f.query_timestamp),
            time_span_start: chrono::Utc.from_utc_datetime(
                &f.time_span_start
                    .and_hms_opt(0, 0, 0)
                    .expect("invalid time"),
            ),
            time_span_end: chrono::Utc
                .from_utc_datetime(&f.time_span_end.and_hms_opt(0, 0, 0).expect("invalid time")),
            measurement_rate: f.measurement_rate,
            measurements: val
                .measurements
                .into_iter()
                .map(|m| ForecastMeasurement {
                    id: Some(m.id),
                    time: chrono::Utc.from_utc_datetime(&m.time),
                    values: m.value,
                })
                .collect(),
        })
    }
}
impl FromJson<models::Forecast> for NestedForecastInsert {
    fn from_json(j: models::Forecast) -> Result<Self, String> {
        let forecast_insert = ForecastInsert {
            group_id: j.group_id,
            station_id: j.station_id,
            station: j.station,
            field_ids: j.field_ids,
            model_id: j.model_id,
            model: j.model,
            query_timestamp: j.query_timestamp.naive_local(),
            time_span_start: j.time_span_start.naive_local().date(),
            time_span_end: j.time_span_end.naive_local().date(),
            measurement_rate: j.measurement_rate,
        };

        let measurements_insert = j
            .measurements
            .into_iter()
            .map(|m| (m.time.naive_local(), m.values))
            .collect();

        Ok(NestedForecastInsert::new(
            forecast_insert,
            measurements_insert,
        ))
    }
}
