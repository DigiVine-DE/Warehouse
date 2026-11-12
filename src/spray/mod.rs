use crate::{
    geo_conv::{DieselConv2D, JsonConv},
    response::{Delete, FromEntry, FromJson, Get, Post, Search},
};

use digivine_api_postgres::spray::{
    NestedSprayComplete, NestedSprayCompleteInsert, NestedSprayTask, NestedSprayTaskInsert,
    SprayCompletePersistent, SprayPlantProtectionInsert, SprayPlantProtectionPersistent,
    SprayTaskPersistent,
};
use digivine_server::{
    models::{self, SprayTask, SprayTaskProperties, SprayTaskPropertiesPlantProtection},
    DeleteOneCompletedSprayTaskResponse, DeleteOneScheduledSprayTaskResponse,
    GetAllCompletedSprayTasksResponse, GetAllScheduledSprayTasksResponse,
    GetOneCompletedSprayTaskResponse, GetOneScheduledSprayTaskResponse,
    PostCompletedSprayTaskResponse, PostScheduledSprayTaskResponse,
};

impl Search<NestedSprayComplete, models::SprayTask> for GetAllCompletedSprayTasksResponse {}
impl Get<NestedSprayComplete, models::SprayTask> for GetOneCompletedSprayTaskResponse {}
impl Post<NestedSprayCompleteInsert, models::SprayTask> for PostCompletedSprayTaskResponse {}
impl Delete<NestedSprayComplete> for DeleteOneCompletedSprayTaskResponse {}

impl Search<NestedSprayTask, models::SprayTask> for GetAllScheduledSprayTasksResponse {}
impl Get<NestedSprayTask, models::SprayTask> for GetOneScheduledSprayTaskResponse {}
impl Post<NestedSprayTaskInsert, models::SprayTask> for PostScheduledSprayTaskResponse {}
impl Delete<NestedSprayTask> for DeleteOneScheduledSprayTaskResponse {}

impl FromJson<models::SprayTask> for NestedSprayTaskInsert {
    fn from_json(val: models::SprayTask) -> Result<Self, String> {
        let props = val.properties;
        Ok(NestedSprayTaskInsert {
            spray: SprayTaskPersistent {
                task_id: props.task_id,
                task_name: props.task_name,
                customer_name: props.customer_name,
                farm_name: props.farm_name,
                fruit_group_name: props.fruit_group_name,
                fruit_name: props.fruit_name,
                plain_name: props.plain_name,
                plain_size: props.plain_size,
                geom: val.geometry.try_to_diesel()?,
            },
            protections: props
                .plant_protection
                .into_iter()
                .map(|m| to_protection_insert(m, Some(props.task_id), None))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl FromJson<models::SprayTask> for NestedSprayCompleteInsert {
    fn from_json(val: models::SprayTask) -> Result<Self, String> {
        let props = val.properties;
        Ok(NestedSprayCompleteInsert {
            spray: SprayCompletePersistent {
                task_id: props.task_id,
                task_name: props.task_name,
                customer_name: props.customer_name,
                farm_name: props.farm_name,
                fruit_group_name: props.fruit_group_name,
                fruit_name: props.fruit_name,
                plain_name: props.plain_name,
                plain_size: props.plain_size,
                application_total_volume: props.application_total_volume,
                total_area: props.total_area,
                effective_total_distance: props.effective_total_distance,
                effective_total_time: props.effective_total_time,
                geom: val.geometry.try_to_diesel()?,
            },
            protections: props
                .plant_protection
                .into_iter()
                .map(|m| to_protection_insert(m, None, Some(props.task_id)))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl FromEntry<NestedSprayTask> for models::SprayTask {
    fn from_entry(entry: NestedSprayTask) -> Result<Self, String> {
        Ok(SprayTask {
            type_: "Feature".to_string(),
            geometry: entry.spray.geom.try_to_json()?,
            properties: SprayTaskProperties {
                plant_protection: Vec::from_entry(entry.protections)?,
                task_id: entry.spray.task_id,
                task_name: entry.spray.task_name,
                customer_name: entry.spray.customer_name,
                farm_name: entry.spray.farm_name,
                fruit_group_name: entry.spray.fruit_group_name,
                fruit_name: entry.spray.fruit_name,
                plain_name: entry.spray.plain_name,
                plain_size: entry.spray.plain_size,
                application_total_volume: None,
                total_area: None,
                effective_total_distance: None,
                effective_total_time: None,
            },
        })
    }
}

impl FromEntry<NestedSprayComplete> for models::SprayTask {
    fn from_entry(entry: NestedSprayComplete) -> Result<Self, String> {
        Ok(SprayTask {
            type_: "Feature".to_string(),
            geometry: entry.spray.geom.try_to_json()?,
            properties: SprayTaskProperties {
                plant_protection: Vec::from_entry(entry.protections)?,
                task_id: entry.spray.task_id,
                task_name: entry.spray.task_name,
                customer_name: entry.spray.customer_name,
                farm_name: entry.spray.farm_name,
                fruit_group_name: entry.spray.fruit_group_name,
                fruit_name: entry.spray.fruit_name,
                plain_name: entry.spray.plain_name,
                plain_size: entry.spray.plain_size,
                application_total_volume: entry.spray.application_total_volume,
                total_area: entry.spray.total_area,
                effective_total_distance: entry.spray.effective_total_distance,
                effective_total_time: entry.spray.effective_total_time,
            },
        })
    }
}

impl FromEntry<Vec<SprayPlantProtectionPersistent>>
    for Vec<models::SprayTaskPropertiesPlantProtection>
{
    fn from_entry(e: Vec<SprayPlantProtectionPersistent>) -> Result<Self, String> {
        Ok(e.into_iter()
            .map(|m| SprayTaskPropertiesPlantProtection {
                amount: Some(m.amount),
                name: Some(m.name),
                unit: Some(m.unit),
            })
            .collect())
    }
}

fn to_protection_insert(
    val: models::SprayTaskPropertiesPlantProtection,
    task_id: Option<i64>,
    complete_id: Option<i64>,
) -> Result<SprayPlantProtectionInsert, String> {
    match (val.amount, val.name, val.unit) {
        (Some(amount), Some(name), Some(unit)) => Ok(SprayPlantProtectionInsert {
            name,
            amount,
            unit,
            task_id,
            complete_id,
        }),
        (None, _, _) => Err("the amount of SprayTaskPropertiesPlantProtection must be set".into()),
        (_, None, _) => Err("the name of SprayTaskPropertiesPlantProtection must be set".into()),
        (_, _, None) => Err("the unit of SprayTaskPropertiesPlantProtection must be set".into()),
    }
}
