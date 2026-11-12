use crate::{
    geo_conv::{DieselConv2D, JsonConv},
    response::{Delete, FromEntry, FromJson, Get, Post, Search},
};
use chrono::TimeZone;
use digivine_api_postgres::quality_measurement::db::{
    QualityMeasurementInsert, QualityMeasurementPersistent,
};
use digivine_server::{
    models::{self, QualityMeasurement, QualityMeasurementProperties},
    DeleteOneQualityMeasurementResponse, GetAllQualityMeasurementsResponse,
    GetOneQualityMeasurementResponse, PostQualityMeasurementResponse,
};
use uuid::Uuid;

impl Search<QualityMeasurementPersistent, models::QualityMeasurement>
    for GetAllQualityMeasurementsResponse
{
}
impl Get<QualityMeasurementPersistent, models::QualityMeasurement>
    for GetOneQualityMeasurementResponse
{
}
impl Delete<QualityMeasurementPersistent> for DeleteOneQualityMeasurementResponse {}
impl Post<QualityMeasurementInsert, (i64, models::QualityMeasurement)>
    for PostQualityMeasurementResponse
{
}

impl FromEntry<QualityMeasurementPersistent> for models::QualityMeasurement {
    fn from_entry(p: QualityMeasurementPersistent) -> Result<Self, String> {
        Ok(QualityMeasurement {
            type_: "Feature".to_owned(),
            geometry: p.geo_point.try_to_json()?,
            properties: QualityMeasurementProperties {
                id: Some(p.id),
                plant_id: p.plant_id,
                timestamp: chrono::Utc.from_utc_datetime(&p.timestamp),
                device: p.device,
                measurement_uuid: Uuid::parse_str(&p.measurement_uuid)
                    .map_err(|e| e.to_string())?,
                malic_acid: p.malic_acid,
                fructose: p.fructose,
                glucose: p.glucose,
                tartaric_acid: Some(p.tartaric_acid),
            },
        })
    }
}

impl FromJson<(i64, models::QualityMeasurement)> for QualityMeasurementInsert {
    fn from_json((plant_id, entry): (i64, models::QualityMeasurement)) -> Result<Self, String> {
        Ok(Self {
            plant_id,
            timestamp: entry.properties.timestamp.naive_local(),
            device: entry.properties.device,
            measurement_uuid: entry.properties.measurement_uuid.to_string(),
            tartaric_acid: entry.properties.tartaric_acid.unwrap(),
            malic_acid: entry.properties.malic_acid,
            fructose: entry.properties.fructose,
            glucose: entry.properties.glucose,
            geo_point: entry.geometry.try_to_diesel()?,
        })
    }
}
