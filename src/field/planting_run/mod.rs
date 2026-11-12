use crate::response::{Delete, FromEntry, FromJson, Get, Post, Search};
use diesel_ext::GeoBBox2D;
use digivine_api_postgres::field::planting_run::db::{
    NestedPlantingRun, NestedPlantingRunInsert, PlantingRunInsert,
};
use digivine_server::{
    models::{self, PlanterRun, PlanterRunProperties},
    DeleteOnePlanterRunResponse, GetAllPlanterRunsResponse, GetOnePlanterRunResponse,
    PostPlanterRunResponse,
};
use std::convert::TryFrom;

pub mod plant;

impl Search<NestedPlantingRun, models::PlanterRun> for GetAllPlanterRunsResponse {}
impl Get<NestedPlantingRun, models::PlanterRun> for GetOnePlanterRunResponse {}
impl Post<NestedPlantingRunInsert, models::PlanterRun> for PostPlanterRunResponse {}
impl Delete<NestedPlantingRun> for DeleteOnePlanterRunResponse {}

impl FromEntry<NestedPlantingRun> for models::PlanterRun {
    fn from_entry(val: NestedPlantingRun) -> Result<Self, String> {
        let plants = val.plants;
        let f = val.planting_run;
        Ok(PlanterRun {
            type_: "FeatureCollection".to_owned(),
            features: plants
                .into_iter()
                .map(FromEntry::from_entry)
                .collect::<Result<_, _>>()?,
            bbox: f.bounding_box.map(|b| b.into()),
            properties: PlanterRunProperties {
                id: Some(f.id),
                user_id: f.user_id,
                terminal_id: f.terminal_id,
                field_id: f.field_id,
            },
        })
    }
}

impl FromJson<models::PlanterRun> for NestedPlantingRunInsert {
    fn from_json(run: models::PlanterRun) -> Result<Self, String> {
        Ok(NestedPlantingRunInsert {
            planting_run_insert: PlantingRunInsert {
                user_id: run.properties.user_id,
                terminal_id: run.properties.terminal_id,
                field_id: run.properties.field_id,
                bounding_box: run
                    .bbox
                    .map(GeoBBox2D::try_from)
                    .transpose()?
                    .map(Into::into),
            },
            plants_insert: run
                .features
                .into_iter()
                .map(|e| (0, e)) // with fake id as it will be updated while insertion
                .map(FromJson::from_json)
                .collect::<Result<_, _>>()?,
        })
    }
}
