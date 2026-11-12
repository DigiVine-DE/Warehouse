use crate::{
    geo_conv::{DieselConv2D, JsonConv},
    response::{Delete, FromEntry, FromJson, Get, Patch, Post, Search},
};
use chrono::TimeZone;

use digivine_api_postgres::field::planting_run::plant::db::{
    PlantChangeSet, PlantInsert, PlantPersistent,
};
use digivine_server::{
    models::{self, Plant, PlantProperties},
    DeleteOnePlantResponse, GetAllPlantsResponse, GetOnePlantResponse, PatchOnePlantResponse,
    PostPlantResponse,
};

impl Search<PlantPersistent, models::Plant> for GetAllPlantsResponse {}
impl Get<PlantPersistent, models::Plant> for GetOnePlantResponse {}
impl Delete<PlantPersistent> for DeleteOnePlantResponse {}
impl Post<PlantInsert, (i64, models::Plant)> for PostPlantResponse {}
impl Patch<PlantChangeSet, PlantPersistent, models::PlantPatch, models::Plant>
    for PatchOnePlantResponse
{
}

impl FromEntry<PlantPersistent> for models::Plant {
    fn from_entry(p: PlantPersistent) -> Result<Self, String> {
        Ok(Plant {
            type_: "Feature".to_owned(),
            geometry: p.geo_point.try_to_json()?,
            properties: PlantProperties {
                id: Some(p.id),
                import_key: p.import_key,
                timestamp: chrono::Utc.from_utc_datetime(&p.timestamp),
                height: p.height,
                deviation: p.deviation,
                speed: p.speed,
                plant_distance: p.plant_distance,
                row_distance: p.row_distance,
                row_number: p.row_number,
            },
        })
    }
}

impl FromJson<(i64, models::Plant)> for PlantInsert {
    fn from_json((plantrun_id, entry): (i64, models::Plant)) -> Result<Self, String> {
        Ok(PlantInsert {
            planting_run_id: plantrun_id,
            import_key: entry.properties.import_key,
            timestamp: entry.properties.timestamp.naive_local(),
            height: entry.properties.height,
            deviation: entry.properties.deviation,
            speed: entry.properties.speed,
            plant_distance: entry.properties.plant_distance,
            row_distance: entry.properties.row_distance,
            row_number: entry.properties.row_number,
            geo_point: entry.geometry.try_to_diesel()?,
        })
    }
}

impl FromJson<models::PlantPatch> for PlantChangeSet {
    fn from_json(entry: models::PlantPatch) -> Result<Self, String> {
        Ok(PlantChangeSet {
            import_key: entry.import_key,
            timestamp: entry.timestamp.map(|t| t.naive_local()),
            height: entry.height,
            deviation: entry.deviation,
            speed: entry.speed,
            plant_distance: entry.plant_distance,
            row_distance: entry.row_distance,
            row_number: entry.row_number,
        })
    }
}
