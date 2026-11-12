use crate::response::{Delete, FromEntry, FromJson, Get, Patch, Post, Search};
use digivine_api_vivc::db::{CultivarFilter, VivcChangeSet, VivcInsert, VivcPersistent};
use digivine_server::{
    models, DeleteOneCultivarResponse, GetAllCultivarsResponse, GetOneCultivarResponse,
    PatchOneCultivarResponse, PostCultivarResponse,
};
impl Get<VivcPersistent, models::Cultivar> for GetOneCultivarResponse {}
impl Search<VivcPersistent, models::Cultivar> for GetAllCultivarsResponse {}
impl Post<VivcInsert, models::Cultivar> for PostCultivarResponse {}
impl Delete<VivcPersistent> for DeleteOneCultivarResponse {}
impl Patch<VivcChangeSet, VivcPersistent, models::CultivarPatch, models::Cultivar>
    for PatchOneCultivarResponse
{
}

impl FromEntry<VivcPersistent> for models::Cultivar {
    fn from_entry(e: VivcPersistent) -> Result<Self, String> {
        Ok(Self {
            id: Some(e.id),
            cultivar_name: e.cultivar_name,
            prime_name: e.prime_name,
            variety_number: e.variety_number,
            species: e.species,
            color: e.color,
            country_or_region: e.country_or_region,
        })
    }
}

impl FromJson<models::Cultivar> for VivcInsert {
    fn from_json(e: models::Cultivar) -> Result<Self, String> {
        Ok(Self {
            cultivar_name: e.cultivar_name,
            prime_name: e.prime_name,
            variety_number: e.variety_number,
            species: e.species,
            color: e.color,
            country_or_region: e.country_or_region,
        })
    }
}

impl<'a> FromJson<CultivarFilter<'a>> for CultivarFilter<'a> {
    fn from_json(e: CultivarFilter<'a>) -> Result<Self, String> {
        Ok(e)
    }
}

impl FromJson<models::CultivarPatch> for VivcChangeSet {
    fn from_json(e: models::CultivarPatch) -> Result<Self, String> {
        Ok(Self {
            cultivar_name: e.cultivar_name,
            prime_name: e.prime_name,
            variety_number: e.variety_number,
            species: e.species,
            color: e.color,
            country_or_region: e.country_or_region,
        })
    }
}
