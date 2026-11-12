use crate::{
    geo_conv::DieselConv3D,
    response::{Delete, FromEntry, Get, GetAll},
};
use digivine_api_terrain::NestedTerrain;
use digivine_server::{
    models::{self, Terrain, TerrainProperties},
    DeleteOneTerrainResponse, GetAllTerrainsResponse, GetOneTerrainResponse,
};
use postgis_diesel::gps::MultiPointZ;

impl FromEntry<NestedTerrain> for models::Terrain {
    fn from_entry(e: NestedTerrain) -> Result<Self, String> {
        let meta = e.meta;
        let multi_point: MultiPointZ = e.data.into_iter().map(|point| point.geo_point_z).collect();
        Ok(Terrain {
            type_: "Feature".to_owned(),
            id: meta.id,
            geometry: multi_point.try_to_json()?,
            properties: TerrainProperties {
                description: meta.description,
                field_id: meta.field_id,
            },
        })
    }
}
impl Get<NestedTerrain, models::Terrain> for GetOneTerrainResponse {}
impl GetAll<NestedTerrain, models::Terrain> for GetAllTerrainsResponse {}
impl Delete<NestedTerrain> for DeleteOneTerrainResponse {}
