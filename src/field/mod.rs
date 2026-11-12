pub mod planting_run;

use crate::{
    geo_conv::{DieselConv2D, JsonConv},
    response::{Delete, FromEntry, FromJson, Get, Patch, Post, Search},
};

use digivine_api_postgres::field::db::{
    FieldChangeSet, FieldInsert, FieldPersistent, NestedField, NestedFieldInsert,
};
use digivine_server::{
    models::{self, Field, FieldProperties},
    DeleteOneFieldResponse, GetAllFieldsResponse, GetOneFieldResponse, PatchOneFieldResponse,
    PostFieldResponse,
};

impl Search<NestedField, models::Field> for GetAllFieldsResponse {}
impl Get<NestedField, models::Field> for GetOneFieldResponse {}
impl Post<NestedFieldInsert, models::Field> for PostFieldResponse {}
impl Delete<NestedField> for DeleteOneFieldResponse {}
impl Patch<FieldChangeSet, FieldPersistent, models::FieldPatch, models::Field>
    for PatchOneFieldResponse
{
}

impl FromEntry<NestedField> for models::Field {
    fn from_entry(val: NestedField) -> Result<Self, String> {
        let mut f = Self::from_entry(val.field)?;
        f.planter_runs = val
            .planting_runs
            .into_iter()
            .map(FromEntry::from_entry)
            .collect::<Result<_, _>>()?;
        Ok(f)
    }
}

impl FromEntry<FieldPersistent> for models::Field {
    fn from_entry(f: FieldPersistent) -> Result<Self, String> {
        Ok(Field {
            type_: "Feature".to_owned(),
            geometry: f.geom.map(|x| x.try_to_json()).transpose()?,
            planter_runs: vec![],
            properties: FieldProperties {
                id: Some(f.id),
                name: f.name,
                station_id: f.station_id,
                number: f.field_number,
                size: f.field_size,
                fruit_name: f.fruit_name,
                historical_station_id: f.historical_station_id,
            },
        })
    }
}

impl FromJson<models::Field> for NestedFieldInsert {
    fn from_json(field: models::Field) -> Result<Self, String> {
        Ok(NestedFieldInsert {
            field_insert: FieldInsert {
                name: field.properties.name.clone(),
                station_id: field.properties.station_id,
                geom: field
                    .geometry
                    .map(|x| x.try_to_diesel_option())
                    .transpose()?
                    .flatten(),
                field_number: field.properties.number,
                field_size: field.properties.size,
                fruit_name: field.properties.fruit_name,
            },
            planting_runs_inserts: field
                .planter_runs
                .into_iter()
                .map(FromJson::from_json)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl FromJson<models::FieldPatch> for FieldChangeSet {
    fn from_json(entry: models::FieldPatch) -> Result<Self, String> {
        Ok(FieldChangeSet {
            name: entry.name,
            station_id: entry.station_id,
            historical_station_id: entry.historical_station_id,
            field_number: entry.number,
            field_size: entry.size,
            fruit_name: entry.fruit_name,
        })
    }
}
