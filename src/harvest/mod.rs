use crate::{
    geo_conv::{DieselConv2D, JsonConv},
    response::{Delete, FromEntry, FromJson, Get, Post, Search},
};

use digivine_api_postgres::harvest::{
    harvest_complete::{
        HarvestCompletePersistent, HarvestMeasurementInsert, HarvestMeasurementPersistent,
        NestedHarvestComplete, NestedHarvestCompleteInsert,
    },
    harvest_task::HarvestTaskPersistent,
};
use digivine_server::{
    models::{
        HarvestMeasurement, HarvestMeasurementProperties, HarvestOrTask, HarvestProperties,
        HarvestTask, HarvestTaskProperties, Polygon,
    },
    DeleteOneCompletedHarvestTaskResponse, DeleteOneScheduledHarvestTaskResponse,
    GetAllCompletedHarvestTasksResponse, GetAllScheduledHarvestTasksResponse,
    GetOneCompletedHarvestTaskResponse, GetOneScheduledHarvestTaskResponse,
    PostCompletedHarvestTaskResponse, PostScheduledHarvestTaskResponse,
};

impl Get<HarvestTaskPersistent, HarvestTask> for GetOneScheduledHarvestTaskResponse {}
impl Search<HarvestTaskPersistent, HarvestTask> for GetAllScheduledHarvestTasksResponse {}
impl Post<HarvestTaskPersistent, HarvestTask> for PostScheduledHarvestTaskResponse {}
impl Delete<HarvestTaskPersistent> for DeleteOneScheduledHarvestTaskResponse {}

impl Get<NestedHarvestComplete, HarvestOrTask> for GetOneCompletedHarvestTaskResponse {}
impl Search<NestedHarvestComplete, HarvestOrTask> for GetAllCompletedHarvestTasksResponse {}
impl Post<NestedHarvestCompleteInsert, HarvestOrTask> for PostCompletedHarvestTaskResponse {}
impl Delete<NestedHarvestComplete> for DeleteOneCompletedHarvestTaskResponse {}

impl FromEntry<HarvestTaskPersistent> for HarvestTask {
    fn from_entry(harvest: HarvestTaskPersistent) -> Result<Self, String> {
        Ok(HarvestTask {
            type_: "Feature".to_string(),
            geometry: harvest.geom.try_to_json()?,
            properties: HarvestTaskProperties {
                task_id: harvest.task_id,
                task_name: harvest.task_name,
                customer_name: harvest.customer_name,
                farm_name: harvest.farm_name,
                fruit_group_name: harvest.fruit_group_name,
                fruit_name: harvest.fruit_name,
                plain_name: harvest.plain_name,
                plain_size: harvest.plain_size,
                yield_total_mass: harvest.yield_total_mass,
                total_area: harvest.total_area,
                effective_total_distance: harvest.effective_total_distance,
                effective_total_time: harvest.effective_total_time,
            },
        })
    }
}

impl FromJson<HarvestTask> for HarvestTaskPersistent {
    fn from_json(val: HarvestTask) -> Result<Self, String> {
        Ok(HarvestTaskPersistent {
            task_id: val.properties.task_id,
            task_name: val.properties.task_name,
            customer_name: val.properties.customer_name,
            farm_name: val.properties.farm_name,
            fruit_group_name: val.properties.fruit_group_name,
            fruit_name: val.properties.fruit_name,
            plain_name: val.properties.plain_name,
            plain_size: val.properties.plain_size,
            yield_total_mass: val.properties.yield_total_mass,
            total_area: val.properties.total_area,
            effective_total_distance: val.properties.effective_total_distance,
            effective_total_time: val.properties.effective_total_time,
            geom: val.geometry.try_to_diesel()?,
        })
    }
}

impl FromEntry<HarvestMeasurementPersistent> for HarvestMeasurement {
    fn from_entry(measurement: HarvestMeasurementPersistent) -> Result<Self, String> {
        Ok(HarvestMeasurement {
            type_: "Feature".to_string(),
            geometry: measurement.geom.unwrap_or_default().try_to_json()?,
            properties: HarvestMeasurementProperties {
                acid: measurement.acid,
                sugar: measurement.sugar,
                spectral: measurement.spectral,
            },
        })
    }
}

impl FromJson<HarvestMeasurement> for HarvestMeasurementInsert {
    fn from_json(val: HarvestMeasurement) -> Result<Self, String> {
        Ok(HarvestMeasurementInsert {
            task_id: 0,
            acid: val.properties.acid,
            sugar: val.properties.sugar,
            spectral: val.properties.spectral,
            geom: val.geometry.try_to_diesel_option()?,
        })
    }
}

impl FromEntry<NestedHarvestComplete> for HarvestOrTask {
    fn from_entry(harvest: NestedHarvestComplete) -> Result<Self, String> {
        let geom: Option<Polygon> = harvest.harvest.geom.map(|x| x.try_to_json()).transpose()?;

        let measurements = match geom {
            None => Some(
                harvest
                    .measurements
                    .into_iter()
                    .map(HarvestMeasurement::from_entry)
                    .collect::<Result<_, _>>()?,
            ),
            Some(_) => None,
        };
        Ok(HarvestOrTask {
            type_: match geom.is_none() {
                true => "FeatureCollection".to_string(),
                false => "Feature".to_string(),
            },
            geometry: geom,
            features: measurements,
            properties: HarvestProperties {
                task_id: harvest.harvest.task_id,
                task_name: harvest.harvest.task_name,
                customer_name: harvest.harvest.customer_name,
                farm_name: harvest.harvest.farm_name,
                fruit_group_name: harvest.harvest.fruit_group_name,
                fruit_name: harvest.harvest.fruit_name,
                plain_name: harvest.harvest.plain_name,
                plain_size: harvest.harvest.plain_size,
                yield_total_mass: harvest.harvest.yield_total_mass,
                total_area: harvest.harvest.total_area,
                effective_total_distance: harvest.harvest.effective_total_distance,
                effective_total_time: harvest.harvest.effective_total_time,
                average_tartaric_acid: harvest.harvest.average_tartaric_acid,
                average_malic_acid: harvest.harvest.average_malic_acid,
                average_glucose: harvest.harvest.average_glucose,
                average_fructose: harvest.harvest.average_fructose,
            },
        })
    }
}

impl FromJson<HarvestOrTask> for NestedHarvestCompleteInsert {
    fn from_json(value: HarvestOrTask) -> Result<Self, String> {
        let geom = value
            .geometry
            .map(|x| x.try_to_diesel_option())
            .transpose()?
            .flatten();

        let measurements: Vec<_> = match value.features {
            Some(m) => m
                .into_iter()
                .map(HarvestMeasurementInsert::from_json)
                .collect::<Result<_, _>>()?,
            None => vec![],
        };

        Ok(NestedHarvestCompleteInsert {
            harvest: HarvestCompletePersistent {
                task_id: value.properties.task_id,
                task_name: value.properties.task_name,
                customer_name: value.properties.customer_name,
                farm_name: value.properties.farm_name,
                fruit_group_name: value.properties.fruit_group_name,
                fruit_name: value.properties.fruit_name,
                plain_name: value.properties.plain_name,
                plain_size: value.properties.plain_size,
                yield_total_mass: value.properties.yield_total_mass,
                total_area: value.properties.total_area,
                effective_total_distance: value.properties.effective_total_distance,
                effective_total_time: value.properties.effective_total_time,
                average_tartaric_acid: value.properties.average_tartaric_acid,
                average_malic_acid: value.properties.average_malic_acid,
                average_glucose: value.properties.average_glucose,
                average_fructose: value.properties.average_fructose,
                geom,
            },
            measurements,
        })
    }
}
