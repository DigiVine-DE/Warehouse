use crate::response::{Delete, FromEntry, FromJson, Get, Patch, Post, Search};
use digivine_api_postgres::weather::{
    WeatherStation, WeatherStationChangeSet, WeatherStationFilter,
};
use digivine_server::{
    models, DeleteOneWeatherStationResponse, GetAllWeatherStationsResponse,
    GetOneWeatherStationResponse, PatchOneWeatherStationResponse, PostWeatherStationResponse,
};

impl Get<WeatherStation, models::WeatherStation> for GetOneWeatherStationResponse {}
impl Post<WeatherStation, models::WeatherStation> for PostWeatherStationResponse {}
impl Search<WeatherStation, models::WeatherStation> for GetAllWeatherStationsResponse {}
impl Delete<WeatherStation> for DeleteOneWeatherStationResponse {}
impl
    Patch<
        WeatherStationChangeSet,
        WeatherStation,
        models::WeatherStationPatch,
        models::WeatherStation,
    > for PatchOneWeatherStationResponse
{
}

impl FromEntry<WeatherStation> for models::WeatherStation {
    fn from_entry(e: WeatherStation) -> Result<Self, String> {
        Ok(Self {
            type_: "Feature".to_string(),
            geometry: FromEntry::from_entry(e.station_geo_point_z)?,
            properties: models::WeatherStationProperties {
                id: e.station_id,
                name: e.station_name,
                bundesland: e.station_bundesland,
            },
        })
    }
}

impl FromJson<models::WeatherStation> for WeatherStation {
    fn from_json(e: models::WeatherStation) -> Result<Self, String> {
        Ok(Self {
            station_id: e.properties.id,
            station_name: e.properties.name,
            station_bundesland: e.properties.bundesland,
            station_geo_point_z: FromJson::from_json(e.geometry)?,
        })
    }
}

impl FromJson<models::WeatherStationPatch> for WeatherStationChangeSet {
    fn from_json(e: models::WeatherStationPatch) -> Result<Self, String> {
        Ok(Self {
            station_name: e.station_name,
            station_bundesland: e.station_bundesland,
            station_geo_point_z: FromJson::from_json(e.geometry)?,
        })
    }
}

impl<'a> FromJson<WeatherStationFilter<'a>> for WeatherStationFilter<'a> {
    fn from_json(f: WeatherStationFilter<'a>) -> Result<Self, String> {
        Ok(f)
    }
}
