use crate::response::{date_to_str, Delete, FromEntry, FromJson, Get, Patch, Post, Search};
use chrono::NaiveDate;
use digivine_api_postgres::weather::{WeatherData, WeatherDataChangeSet, WeatherDataFilter};
use digivine_server::{
    models::{
        self, WeatherDataFm, WeatherDataFx, WeatherDataNm, WeatherDataPm, WeatherDataRsk,
        WeatherDataSdk, WeatherDataShkTag, WeatherDataTgk, WeatherDataTmk, WeatherDataTnk,
        WeatherDataTxk, WeatherDataUpm, WeatherDataVpm,
    },
    DeleteOneWeatherDataResponse, GetAllWeatherDataResponse, GetOneWeatherDataResponse,
    PatchOneWeatherDataResponse, PostWeatherDataResponse,
};

impl Get<WeatherData, models::WeatherData> for GetOneWeatherDataResponse {}
impl Delete<WeatherData> for DeleteOneWeatherDataResponse {}
impl Post<WeatherData, models::WeatherData> for PostWeatherDataResponse {}
impl Patch<WeatherDataChangeSet, WeatherData, models::WeatherDataPatch, models::WeatherData>
    for PatchOneWeatherDataResponse
{
}
impl Search<WeatherData, models::WeatherData> for GetAllWeatherDataResponse {}

macro_rules! quality {
    ($e:expr) => {
        $e.map(|x| x.quality_level as i16)
    };
}

macro_rules! value {
    ($e:expr) => {
        $e.as_ref().map(|r| r.value)
    };
}

impl<'a> FromJson<WeatherDataFilter<'a>> for WeatherDataFilter<'a> {
    fn from_json(e: WeatherDataFilter<'a>) -> Result<Self, String> {
        Ok(e)
    }
}

impl FromJson<models::WeatherData> for WeatherData {
    fn from_json(e: models::WeatherData) -> Result<Self, String> {
        Ok(Self {
            station_id: e.station_id,
            measurement_date: NaiveDate::from_json(e.measurement_date)?,
            fx: value!(e.fx),
            fx_qn: quality!(e.fx),
            fm: value!(e.fm),
            fm_qn: quality!(e.fm),
            rsk: value!(e.rsk),
            rsk_qn: quality!(e.rsk),
            rskf: e
                .rskf
                .map(|x| i32::try_from(x).map_err(|e| e.to_string()))
                .transpose()?,
            sdk: value!(e.sdk),
            sdk_qn: quality!(e.sdk),
            shk_tag: value!(e.shk_tag),
            shk_tag_qn: quality!(e.shk_tag),
            nm: value!(e.nm),
            nm_qn: quality!(e.nm),
            vpm: value!(e.vpm),
            vpm_qn: quality!(e.vpm),
            pm: value!(e.pm),
            pm_qn: quality!(e.pm),
            tmk: value!(e.tmk),
            tmk_qn: quality!(e.tmk),
            upm: value!(e.upm),
            upm_qn: quality!(e.upm),
            txk: value!(e.txk),
            txk_qn: quality!(e.txk),
            tnk: value!(e.tnk),
            tnk_qn: quality!(e.tnk),
            tgk: value!(e.tgk),
            tgk_qn: quality!(e.tgk),
        })
    }
}

impl FromJson<models::WeatherDataPatch> for WeatherDataChangeSet {
    fn from_json(e: models::WeatherDataPatch) -> Result<Self, String> {
        Ok(Self {
            fx: value!(e.fx),
            fx_qn: quality!(e.fx),
            fm: value!(e.fm),
            fm_qn: quality!(e.fm),
            rsk: value!(e.rsk),
            rsk_qn: quality!(e.rsk),
            rskf: e
                .rskf
                .map(|x| i32::try_from(x).map_err(|e| e.to_string()))
                .transpose()?,
            sdk: value!(e.sdk),
            sdk_qn: quality!(e.sdk),
            shk_tag: value!(e.shk_tag),
            shk_tag_qn: quality!(e.shk_tag),
            nm: value!(e.nm),
            nm_qn: quality!(e.nm),
            vpm: value!(e.vpm),
            vpm_qn: quality!(e.vpm),
            pm: value!(e.pm),
            pm_qn: quality!(e.pm),
            tmk: value!(e.tmk),
            tmk_qn: quality!(e.tmk),
            upm: value!(e.upm),
            upm_qn: quality!(e.upm),
            txk: value!(e.txk),
            txk_qn: quality!(e.txk),
            tnk: value!(e.tnk),
            tnk_qn: quality!(e.tnk),
            tgk: value!(e.tgk),
            tgk_qn: quality!(e.tgk),
        })
    }
}

impl FromEntry<WeatherData> for models::WeatherData {
    fn from_entry(e: WeatherData) -> Result<Self, String> {
        Ok(Self {
            station_id: e.station_id,
            measurement_date: date_to_str(e.measurement_date),
            fx: opt_measurement(e.fx, e.fx_qn, WeatherDataFx::new)?,
            fm: opt_measurement(e.fm, e.fm_qn, WeatherDataFm::new)?,
            rsk: opt_measurement(e.rsk, e.rsk_qn, WeatherDataRsk::new)?,
            rskf: e.rskf.map(|x| x as isize),
            sdk: opt_measurement(e.sdk, e.sdk_qn, WeatherDataSdk::new)?,
            shk_tag: opt_measurement(e.shk_tag, e.shk_tag_qn, WeatherDataShkTag::new)?,
            nm: opt_measurement(e.nm, e.nm_qn, WeatherDataNm::new)?,
            vpm: opt_measurement(e.vpm, e.vpm_qn, WeatherDataVpm::new)?,
            pm: opt_measurement(e.pm, e.pm_qn, WeatherDataPm::new)?,
            tmk: opt_measurement(e.tmk, e.tmk_qn, WeatherDataTmk::new)?,
            upm: opt_measurement(e.upm, e.upm_qn, WeatherDataUpm::new)?,
            txk: opt_measurement(e.txk, e.txk_qn, WeatherDataTxk::new)?,
            tnk: opt_measurement(e.tnk, e.tnk_qn, WeatherDataTnk::new)?,
            tgk: opt_measurement(e.tgk, e.tgk_qn, WeatherDataTgk::new)?,
        })
    }
}

fn opt_measurement<T>(
    v: Option<f64>,
    quality: Option<i16>,
    constructor: impl Fn(f64, u32) -> T,
) -> Result<Option<T>, String> {
    match (v, quality) {
        (Some(v), Some(q)) => u32::try_from(q)
            .map(|q| Some(constructor(v, q)))
            .map_err(|e| e.to_string()),
        _ => Ok(None),
    }
}
