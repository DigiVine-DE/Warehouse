use futures::{
    future,
    future::{BoxFuture, FutureExt},
    stream,
    stream::TryStreamExt,
    Stream,
};
use hyper::{
    header::{HeaderName, HeaderValue, CONTENT_TYPE},
    Body, HeaderMap, Request, Response, StatusCode,
};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::{
    error::Error,
    future::Future,
    marker::PhantomData,
    task::{Context, Poll},
};
pub use swagger::auth::Authorization;
use swagger::{auth::Scopes, ApiError, BodyExt, Has, RequestParser, XSpanIdString};
use url::form_urlencoded;

use crate::header;
#[allow(unused_imports)]
use crate::models;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{
    Api, DeleteCompletedHarvestTaskResponse, DeleteCompletedSprayTaskResponse, DeleteFieldResponse,
    DeleteForecastResponse, DeleteOneAccessRightResponse, DeleteOneCompletedHarvestTaskResponse,
    DeleteOneCompletedSprayTaskResponse, DeleteOneCultivarResponse, DeleteOneFieldResponse,
    DeleteOneForecastResponse, DeleteOnePlantResponse, DeleteOnePlanterRunResponse,
    DeleteOneQualityMeasurementResponse, DeleteOneScheduledHarvestTaskResponse,
    DeleteOneScheduledSprayTaskResponse, DeleteOneTerrainResponse, DeleteOneTokenResponse,
    DeleteOneUserResponse, DeleteOneWeatherDataResponse, DeleteOneWeatherStationResponse,
    DeletePlantResponse, DeletePlanterRunResponse, DeleteQualityMeasurementResponse,
    DeleteScheduledHarvestTaskResponse, DeleteScheduledSprayTaskResponse, DeleteTerrainResponse,
    GetAccessRightsResponse, GetAllCompletedHarvestTasksResponse,
    GetAllCompletedSprayTasksResponse, GetAllCultivarsResponse, GetAllFieldsResponse,
    GetAllForecastsResponse, GetAllPlanterRunsResponse, GetAllPlantsResponse,
    GetAllQualityMeasurementsResponse, GetAllScheduledHarvestTasksResponse,
    GetAllScheduledSprayTasksResponse, GetAllTerrainsResponse, GetAllTokensResponse,
    GetAllWeatherDataResponse, GetAllWeatherStationsResponse, GetOneAccessRightResponse,
    GetOneCompletedHarvestTaskResponse, GetOneCompletedSprayTaskResponse, GetOneCultivarResponse,
    GetOneFieldResponse, GetOneForecastResponse, GetOnePlantResponse, GetOnePlanterRunResponse,
    GetOneQualityMeasurementResponse, GetOneScheduledHarvestTaskResponse,
    GetOneScheduledSprayTaskResponse, GetOneTerrainResponse, GetOneWeatherDataResponse,
    GetOneWeatherStationResponse, GetUserTokensResponse, PatchOneCultivarResponse,
    PatchOneFieldResponse, PatchOnePlantResponse, PatchOneWeatherDataResponse,
    PatchOneWeatherStationResponse, PostAccessRightResponse, PostCompletedHarvestTaskResponse,
    PostCompletedSprayTaskResponse, PostCultivarResponse, PostFieldResponse, PostForecastResponse,
    PostPlantResponse, PostPlanterRunResponse, PostQualityMeasurementResponse,
    PostScheduledHarvestTaskResponse, PostScheduledSprayTaskResponse, PostTokenResponse,
    PostUserTokenResponse, PostWeatherDataResponse, PostWeatherStationResponse,
};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/api/v0.3/field$",
            r"^/api/v0.3/field/(?P<FieldId>[^/?#]*)$",
            r"^/api/v0.3/field/(?P<FieldId>[^/?#]*)/planter$",
            r"^/api/v0.3/field/(?P<FieldId>[^/?#]*)/planter/(?P<PlanterRunId>[^/?#]*)$",
            r"^/api/v0.3/field/(?P<FieldId>[^/?#]*)/planter/(?P<PlanterRunId>[^/?#]*)/plant$",
            r"^/api/v0.3/field/(?P<FieldId>[^/?#]*)/planter/(?P<PlanterRunId>[^/?#]*)/plant/(?P<PlantId>[^/?#]*)$",
            r"^/api/v0.3/field/(?P<FieldId>[^/?#]*)/planter/(?P<PlanterRunId>[^/?#]*)/plant/(?P<PlantId>[^/?#]*)/quality_measurement$",
            r"^/api/v0.3/field/(?P<FieldId>[^/?#]*)/planter/(?P<PlanterRunId>[^/?#]*)/plant/(?P<PlantId>[^/?#]*)/quality_measurement/(?P<QualityMeasurementId>[^/?#]*)$",
            r"^/api/v0.3/forecast$",
            r"^/api/v0.3/forecast/(?P<ForecastId>[^/?#]*)$",
            r"^/api/v0.3/harvest/complete$",
            r"^/api/v0.3/harvest/complete/(?P<HarvestTaskId>[^/?#]*)$",
            r"^/api/v0.3/harvest/task$",
            r"^/api/v0.3/harvest/task/(?P<HarvestTaskId>[^/?#]*)$",
            r"^/api/v0.3/spray/complete$",
            r"^/api/v0.3/spray/complete/(?P<SprayTaskId>[^/?#]*)$",
            r"^/api/v0.3/spray/task$",
            r"^/api/v0.3/spray/task/(?P<SprayTaskId>[^/?#]*)$",
            r"^/api/v0.3/terrain$",
            r"^/api/v0.3/terrain/(?P<TerrainId>[^/?#]*)$",
            r"^/api/v0.3/token$",
            r"^/api/v0.3/token/(?P<UserId>[^/?#]*)$",
            r"^/api/v0.3/token/(?P<UserId>[^/?#]*)/(?P<Tokenstr>[^/?#]*)$",
            r"^/api/v0.3/token/(?P<UserId>[^/?#]*)/(?P<Tokenstr>[^/?#]*)/access_right$",
            r"^/api/v0.3/token/(?P<UserId>[^/?#]*)/(?P<Tokenstr>[^/?#]*)/access_right/(?P<AccessRightPath>[^/?#]*)$",
            r"^/api/v0.3/vivc$",
            r"^/api/v0.3/vivc/(?P<CultivarId>[^/?#]*)$",
            r"^/api/v0.3/weather/data$",
            r"^/api/v0.3/weather/data/(?P<WeatherStationId>[^/?#]*)/(?P<MeasurementDate>[^/?#]*)$",
            r"^/api/v0.3/weather/station$",
            r"^/api/v0.3/weather/station/(?P<WeatherStationId>[^/?#]*)$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_FIELD: usize = 0;
    pub(crate) static ID_FIELD_FIELDID: usize = 1;
    lazy_static! {
        pub static ref REGEX_FIELD_FIELDID: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/field/(?P<FieldId>[^/?#]*)$")
                .expect("Unable to create regex for FIELD_FIELDID");
    }
    pub(crate) static ID_FIELD_FIELDID_PLANTER: usize = 2;
    lazy_static! {
        pub static ref REGEX_FIELD_FIELDID_PLANTER: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/field/(?P<FieldId>[^/?#]*)/planter$")
                .expect("Unable to create regex for FIELD_FIELDID_PLANTER");
    }
    pub(crate) static ID_FIELD_FIELDID_PLANTER_PLANTERRUNID: usize = 3;
    lazy_static! {
        pub static ref REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID: regex::Regex = regex::Regex::new(
            r"^/api/v0.3/field/(?P<FieldId>[^/?#]*)/planter/(?P<PlanterRunId>[^/?#]*)$"
        )
        .expect("Unable to create regex for FIELD_FIELDID_PLANTER_PLANTERRUNID");
    }
    pub(crate) static ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT: usize = 4;
    lazy_static! {
        pub static ref REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT: regex::Regex =
            regex::Regex::new(
                r"^/api/v0.3/field/(?P<FieldId>[^/?#]*)/planter/(?P<PlanterRunId>[^/?#]*)/plant$"
            )
            .expect("Unable to create regex for FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT");
    }
    pub(crate) static ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID: usize = 5;
    lazy_static! {
        pub static ref REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/field/(?P<FieldId>[^/?#]*)/planter/(?P<PlanterRunId>[^/?#]*)/plant/(?P<PlantId>[^/?#]*)$")
                .expect("Unable to create regex for FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID");
    }
    pub(crate) static ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT:
        usize = 6;
    lazy_static! {
        pub static ref REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/field/(?P<FieldId>[^/?#]*)/planter/(?P<PlanterRunId>[^/?#]*)/plant/(?P<PlantId>[^/?#]*)/quality_measurement$")
                .expect("Unable to create regex for FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT");
    }
    pub(crate) static ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT_QUALITYMEASUREMENTID: usize = 7;
    lazy_static! {
        pub static ref REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT_QUALITYMEASUREMENTID: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/field/(?P<FieldId>[^/?#]*)/planter/(?P<PlanterRunId>[^/?#]*)/plant/(?P<PlantId>[^/?#]*)/quality_measurement/(?P<QualityMeasurementId>[^/?#]*)$")
                .expect("Unable to create regex for FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT_QUALITYMEASUREMENTID");
    }
    pub(crate) static ID_FORECAST: usize = 8;
    pub(crate) static ID_FORECAST_FORECASTID: usize = 9;
    lazy_static! {
        pub static ref REGEX_FORECAST_FORECASTID: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/forecast/(?P<ForecastId>[^/?#]*)$")
                .expect("Unable to create regex for FORECAST_FORECASTID");
    }
    pub(crate) static ID_HARVEST_COMPLETE: usize = 10;
    pub(crate) static ID_HARVEST_COMPLETE_HARVESTTASKID: usize = 11;
    lazy_static! {
        pub static ref REGEX_HARVEST_COMPLETE_HARVESTTASKID: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/harvest/complete/(?P<HarvestTaskId>[^/?#]*)$")
                .expect("Unable to create regex for HARVEST_COMPLETE_HARVESTTASKID");
    }
    pub(crate) static ID_HARVEST_TASK: usize = 12;
    pub(crate) static ID_HARVEST_TASK_HARVESTTASKID: usize = 13;
    lazy_static! {
        pub static ref REGEX_HARVEST_TASK_HARVESTTASKID: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/harvest/task/(?P<HarvestTaskId>[^/?#]*)$")
                .expect("Unable to create regex for HARVEST_TASK_HARVESTTASKID");
    }
    pub(crate) static ID_SPRAY_COMPLETE: usize = 14;
    pub(crate) static ID_SPRAY_COMPLETE_SPRAYTASKID: usize = 15;
    lazy_static! {
        pub static ref REGEX_SPRAY_COMPLETE_SPRAYTASKID: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/spray/complete/(?P<SprayTaskId>[^/?#]*)$")
                .expect("Unable to create regex for SPRAY_COMPLETE_SPRAYTASKID");
    }
    pub(crate) static ID_SPRAY_TASK: usize = 16;
    pub(crate) static ID_SPRAY_TASK_SPRAYTASKID: usize = 17;
    lazy_static! {
        pub static ref REGEX_SPRAY_TASK_SPRAYTASKID: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/spray/task/(?P<SprayTaskId>[^/?#]*)$")
                .expect("Unable to create regex for SPRAY_TASK_SPRAYTASKID");
    }
    pub(crate) static ID_TERRAIN: usize = 18;
    pub(crate) static ID_TERRAIN_TERRAINID: usize = 19;
    lazy_static! {
        pub static ref REGEX_TERRAIN_TERRAINID: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/terrain/(?P<TerrainId>[^/?#]*)$")
                .expect("Unable to create regex for TERRAIN_TERRAINID");
    }
    pub(crate) static ID_TOKEN: usize = 20;
    pub(crate) static ID_TOKEN_USERID: usize = 21;
    lazy_static! {
        pub static ref REGEX_TOKEN_USERID: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/token/(?P<UserId>[^/?#]*)$")
                .expect("Unable to create regex for TOKEN_USERID");
    }
    pub(crate) static ID_TOKEN_USERID_TOKENSTR: usize = 22;
    lazy_static! {
        pub static ref REGEX_TOKEN_USERID_TOKENSTR: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/token/(?P<UserId>[^/?#]*)/(?P<Tokenstr>[^/?#]*)$")
                .expect("Unable to create regex for TOKEN_USERID_TOKENSTR");
    }
    pub(crate) static ID_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT: usize = 23;
    lazy_static! {
        pub static ref REGEX_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT: regex::Regex = regex::Regex::new(
            r"^/api/v0.3/token/(?P<UserId>[^/?#]*)/(?P<Tokenstr>[^/?#]*)/access_right$"
        )
        .expect("Unable to create regex for TOKEN_USERID_TOKENSTR_ACCESS_RIGHT");
    }
    pub(crate) static ID_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT_ACCESSRIGHTPATH: usize = 24;
    lazy_static! {
        pub static ref REGEX_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT_ACCESSRIGHTPATH: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/token/(?P<UserId>[^/?#]*)/(?P<Tokenstr>[^/?#]*)/access_right/(?P<AccessRightPath>[^/?#]*)$")
                .expect("Unable to create regex for TOKEN_USERID_TOKENSTR_ACCESS_RIGHT_ACCESSRIGHTPATH");
    }
    pub(crate) static ID_VIVC: usize = 25;
    pub(crate) static ID_VIVC_CULTIVARID: usize = 26;
    lazy_static! {
        pub static ref REGEX_VIVC_CULTIVARID: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/vivc/(?P<CultivarId>[^/?#]*)$")
                .expect("Unable to create regex for VIVC_CULTIVARID");
    }
    pub(crate) static ID_WEATHER_DATA: usize = 27;
    pub(crate) static ID_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE: usize = 28;
    lazy_static! {
        pub static ref REGEX_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/weather/data/(?P<WeatherStationId>[^/?#]*)/(?P<MeasurementDate>[^/?#]*)$")
                .expect("Unable to create regex for WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE");
    }
    pub(crate) static ID_WEATHER_STATION: usize = 29;
    pub(crate) static ID_WEATHER_STATION_WEATHERSTATIONID: usize = 30;
    lazy_static! {
        pub static ref REGEX_WEATHER_STATION_WEATHERSTATIONID: regex::Regex =
            regex::Regex::new(r"^/api/v0.3/weather/station/(?P<WeatherStationId>[^/?#]*)$")
                .expect("Unable to create regex for WEATHER_STATION_WEATHERSTATIONID");
    }
}

pub struct MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData,
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(self.api_impl.clone()))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::empty())
        .expect("Unable to create Method Not Allowed response"))
}

pub struct Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl: api_impl,
            marker: PhantomData,
        }
    }
}

impl<T, C> Clone for Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker.clone(),
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C>
where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future {
        async fn run<T, C>(
            mut api_impl: T,
            req: (Request<Body>, C),
        ) -> Result<Response<Body>, crate::ServiceError>
        where
            T: Api<C> + Clone + Send + 'static,
            C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
        {
            let (request, context) = req;
            let (parts, body) = request.into_parts();
            let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
            let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

            match &method {

            // DeleteOneCultivar - DELETE /vivc/{CultivarId}
            &hyper::Method::DELETE if path.matched(paths::ID_VIVC_CULTIVARID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_VIVC_CULTIVARID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE VIVC_CULTIVARID in set but failed match against \"{}\"", path, paths::REGEX_VIVC_CULTIVARID.as_str())
                    );

                let param_cultivar_id = match percent_encoding::percent_decode(path_params["CultivarId"].as_bytes()).decode_utf8() {
                    Ok(param_cultivar_id) => match param_cultivar_id.parse::<i32>() {
                        Ok(param_cultivar_id) => param_cultivar_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter CultivarId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["CultivarId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_cultivar(
                                            param_cultivar_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOneCultivarResponse::SuccessfullyDeletedCultivar
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOneCultivarResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_CULTIVAR_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneCultivarResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOneCultivarResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOneCultivarResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOneCultivarResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_CULTIVAR_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAllCultivars - GET /vivc
            &hyper::Method::GET if path.matched(paths::ID_VIVC) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_color = query_params.iter().filter(|e| e.0 == "color").map(|e| e.1.to_owned())
                    .filter_map(|param_color| param_color.parse().ok())
                    .collect::<Vec<_>>();
                let param_color = if !param_color.is_empty() {
                    Some(param_color)
                } else {
                    None
                };
                let param_region = query_params.iter().filter(|e| e.0 == "region").map(|e| e.1.to_owned())
                    .filter_map(|param_region| param_region.parse().ok())
                    .collect::<Vec<_>>();
                let param_region = if !param_region.is_empty() {
                    Some(param_region)
                } else {
                    None
                };
                let param_species = query_params.iter().filter(|e| e.0 == "species").map(|e| e.1.to_owned())
                    .filter_map(|param_species| param_species.parse().ok())
                    .collect::<Vec<_>>();
                let param_species = if !param_species.is_empty() {
                    Some(param_species)
                } else {
                    None
                };
                let param_prime_name = query_params.iter().filter(|e| e.0 == "primeName").map(|e| e.1.to_owned())
                    .filter_map(|param_prime_name| param_prime_name.parse().ok())
                    .collect::<Vec<_>>();
                let param_prime_name = if !param_prime_name.is_empty() {
                    Some(param_prime_name)
                } else {
                    None
                };
                let param_cultivar_name = query_params.iter().filter(|e| e.0 == "cultivarName").map(|e| e.1.to_owned())
                    .filter_map(|param_cultivar_name| param_cultivar_name.parse().ok())
                    .collect::<Vec<_>>();
                let param_cultivar_name = if !param_cultivar_name.is_empty() {
                    Some(param_cultivar_name)
                } else {
                    None
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_all_cultivars(
                                            param_color.as_ref(),
                                            param_region.as_ref(),
                                            param_species.as_ref(),
                                            param_prime_name.as_ref(),
                                            param_cultivar_name.as_ref(),
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllCultivarsResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_CULTIVARS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllCultivarsResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_CULTIVARS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllCultivarsResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAllCultivarsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAllCultivarsResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_CULTIVARS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOneCultivar - GET /vivc/{CultivarId}
            &hyper::Method::GET if path.matched(paths::ID_VIVC_CULTIVARID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_VIVC_CULTIVARID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE VIVC_CULTIVARID in set but failed match against \"{}\"", path, paths::REGEX_VIVC_CULTIVARID.as_str())
                    );

                let param_cultivar_id = match percent_encoding::percent_decode(path_params["CultivarId"].as_bytes()).decode_utf8() {
                    Ok(param_cultivar_id) => match param_cultivar_id.parse::<i32>() {
                        Ok(param_cultivar_id) => param_cultivar_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter CultivarId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["CultivarId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_one_cultivar(
                                            param_cultivar_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOneCultivarResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_CULTIVAR_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneCultivarResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_CULTIVAR_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneCultivarResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetOneCultivarResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetOneCultivarResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetOneCultivarResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_CULTIVAR_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PatchOneCultivar - PATCH /vivc/{CultivarId}
            &hyper::Method::PATCH if path.matched(paths::ID_VIVC_CULTIVARID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_VIVC_CULTIVARID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE VIVC_CULTIVARID in set but failed match against \"{}\"", path, paths::REGEX_VIVC_CULTIVARID.as_str())
                    );

                let param_cultivar_id = match percent_encoding::percent_decode(path_params["CultivarId"].as_bytes()).decode_utf8() {
                    Ok(param_cultivar_id) => match param_cultivar_id.parse::<i32>() {
                        Ok(param_cultivar_id) => param_cultivar_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter CultivarId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["CultivarId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_cultivar_patch: Option<models::CultivarPatch> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_cultivar_patch) => param_cultivar_patch,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter CultivarPatch - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter CultivarPatch due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_cultivar_patch = match param_cultivar_patch {
                                    Some(param_cultivar_patch) => param_cultivar_patch,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter CultivarPatch"))
                                                        .expect("Unable to create Bad Request response for missing body parameter CultivarPatch")),
                                };

                                let result = api_impl.patch_one_cultivar(
                                            param_cultivar_id,
                                            param_cultivar_patch,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PatchOneCultivarResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_CULTIVAR_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PatchOneCultivarResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_CULTIVAR_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PatchOneCultivarResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PatchOneCultivarResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PatchOneCultivarResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                PatchOneCultivarResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_CULTIVAR_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter CultivarPatch: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter CultivarPatch")),
                        }
            },

            // PostCultivar - POST /vivc
            &hyper::Method::POST if path.matched(paths::ID_VIVC) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_cultivar: Option<models::Cultivar> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_cultivar) => param_cultivar,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter Cultivar - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter Cultivar due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_cultivar = match param_cultivar {
                                    Some(param_cultivar) => param_cultivar,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter Cultivar"))
                                                        .expect("Unable to create Bad Request response for missing body parameter Cultivar")),
                                };

                                let result = api_impl.post_cultivar(
                                            param_cultivar,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostCultivarResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_CULTIVAR_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostCultivarResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_CULTIVAR_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostCultivarResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostCultivarResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostCultivarResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_CULTIVAR_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Cultivar: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Cultivar")),
                        }
            },

            // DeleteField - DELETE /field
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                                let result = api_impl.delete_field(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteFieldResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_FIELD_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteFieldResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteFieldResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteFieldResponse::MethodNotAllowed
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(405).expect("Unable to turn 405 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteOneField - DELETE /field/{FieldId}
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD_FIELDID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_field(
                                            param_field_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOneFieldResponse::SuccessfullyDeletedField
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOneFieldResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_FIELD_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneFieldResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOneFieldResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOneFieldResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOneFieldResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_FIELD_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAllFields - GET /field
            &hyper::Method::GET if path.matched(paths::ID_FIELD) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_box2d = query_params.iter().filter(|e| e.0 == "box2d").map(|e| e.1.to_owned())
                    .filter_map(|param_box2d| param_box2d.parse().ok())
                    .collect::<Vec<_>>();
                let param_box2d = if !param_box2d.is_empty() {
                    Some(param_box2d)
                } else {
                    None
                };
                let param_fuzzy_field_keywords = query_params.iter().filter(|e| e.0 == "fuzzyFieldKeywords").map(|e| e.1.to_owned())
                    .filter_map(|param_fuzzy_field_keywords| param_fuzzy_field_keywords.parse().ok())
                    .collect::<Vec<_>>();
                let param_fuzzy_field_keywords = if !param_fuzzy_field_keywords.is_empty() {
                    Some(param_fuzzy_field_keywords)
                } else {
                    None
                };
                let param_field_name_contains = query_params.iter().filter(|e| e.0 == "fieldNameContains").map(|e| e.1.to_owned())
                    .filter_map(|param_field_name_contains| param_field_name_contains.parse().ok())
                    .collect::<Vec<_>>();
                let param_field_name_contains = if !param_field_name_contains.is_empty() {
                    Some(param_field_name_contains)
                } else {
                    None
                };
                let param_fruit_name_contains = query_params.iter().filter(|e| e.0 == "fruitNameContains").map(|e| e.1.to_owned())
                    .filter_map(|param_fruit_name_contains| param_fruit_name_contains.parse().ok())
                    .collect::<Vec<_>>();
                let param_fruit_name_contains = if !param_fruit_name_contains.is_empty() {
                    Some(param_fruit_name_contains)
                } else {
                    None
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_all_fields(
                                            param_box2d.as_ref(),
                                            param_fuzzy_field_keywords.as_ref(),
                                            param_field_name_contains.as_ref(),
                                            param_fruit_name_contains.as_ref(),
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllFieldsResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_FIELDS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllFieldsResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_FIELDS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllFieldsResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAllFieldsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAllFieldsResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_FIELDS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOneField - GET /field/{FieldId}
            &hyper::Method::GET if path.matched(paths::ID_FIELD_FIELDID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_one_field(
                                            param_field_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOneFieldResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_FIELD_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneFieldResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_FIELD_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneFieldResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetOneFieldResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetOneFieldResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetOneFieldResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_FIELD_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PatchOneField - PATCH /field/{FieldId}
            &hyper::Method::PATCH if path.matched(paths::ID_FIELD_FIELDID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_field_patch: Option<models::FieldPatch> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_field_patch) => param_field_patch,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter FieldPatch - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter FieldPatch due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_field_patch = match param_field_patch {
                                    Some(param_field_patch) => param_field_patch,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter FieldPatch"))
                                                        .expect("Unable to create Bad Request response for missing body parameter FieldPatch")),
                                };

                                let result = api_impl.patch_one_field(
                                            param_field_id,
                                            param_field_patch,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PatchOneFieldResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_FIELD_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PatchOneFieldResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_FIELD_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PatchOneFieldResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PatchOneFieldResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PatchOneFieldResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                PatchOneFieldResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_FIELD_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter FieldPatch: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter FieldPatch")),
                        }
            },

            // PostField - POST /field
            &hyper::Method::POST if path.matched(paths::ID_FIELD) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_field: Option<models::Field> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_field) => param_field,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter Field - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter Field due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_field = match param_field {
                                    Some(param_field) => param_field,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter Field"))
                                                        .expect("Unable to create Bad Request response for missing body parameter Field")),
                                };

                                let result = api_impl.post_field(
                                            param_field,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostFieldResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_FIELD_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostFieldResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_FIELD_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostFieldResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostFieldResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostFieldResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_FIELD_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Field: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Field")),
                        }
            },

            // DeleteForecast - DELETE /forecast
            &hyper::Method::DELETE if path.matched(paths::ID_FORECAST) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                                let result = api_impl.delete_forecast(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteForecastResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_FORECAST_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteForecastResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteForecastResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteForecastResponse::MethodNotAllowed
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(405).expect("Unable to turn 405 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteOneForecast - DELETE /forecast/{ForecastId}
            &hyper::Method::DELETE if path.matched(paths::ID_FORECAST_FORECASTID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FORECAST_FORECASTID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FORECAST_FORECASTID in set but failed match against \"{}\"", path, paths::REGEX_FORECAST_FORECASTID.as_str())
                    );

                let param_forecast_id = match percent_encoding::percent_decode(path_params["ForecastId"].as_bytes()).decode_utf8() {
                    Ok(param_forecast_id) => match param_forecast_id.parse::<i64>() {
                        Ok(param_forecast_id) => param_forecast_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter ForecastId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["ForecastId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_forecast(
                                            param_forecast_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOneForecastResponse::SuccessfullyDeletedForecast
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOneForecastResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_FORECAST_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneForecastResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOneForecastResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOneForecastResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOneForecastResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_FORECAST_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAllForecasts - GET /forecast
            &hyper::Method::GET if path.matched(paths::ID_FORECAST) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_all_forecasts(
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllForecastsResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_FORECASTS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllForecastsResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_FORECASTS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllForecastsResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAllForecastsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAllForecastsResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_FORECASTS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOneForecast - GET /forecast/{ForecastId}
            &hyper::Method::GET if path.matched(paths::ID_FORECAST_FORECASTID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FORECAST_FORECASTID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FORECAST_FORECASTID in set but failed match against \"{}\"", path, paths::REGEX_FORECAST_FORECASTID.as_str())
                    );

                let param_forecast_id = match percent_encoding::percent_decode(path_params["ForecastId"].as_bytes()).decode_utf8() {
                    Ok(param_forecast_id) => match param_forecast_id.parse::<i64>() {
                        Ok(param_forecast_id) => param_forecast_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter ForecastId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["ForecastId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_one_forecast(
                                            param_forecast_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOneForecastResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_FORECAST_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneForecastResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_FORECAST_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneForecastResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetOneForecastResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetOneForecastResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetOneForecastResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_FORECAST_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PostForecast - POST /forecast
            &hyper::Method::POST if path.matched(paths::ID_FORECAST) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_forecast: Option<models::Forecast> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_forecast) => param_forecast,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter Forecast - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter Forecast due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_forecast = match param_forecast {
                                    Some(param_forecast) => param_forecast,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter Forecast"))
                                                        .expect("Unable to create Bad Request response for missing body parameter Forecast")),
                                };

                                let result = api_impl.post_forecast(
                                            param_forecast,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostForecastResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_FORECAST_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostForecastResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_FORECAST_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostForecastResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostForecastResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostForecastResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_FORECAST_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Forecast: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Forecast")),
                        }
            },

            // DeleteCompletedHarvestTask - DELETE /harvest/complete
            &hyper::Method::DELETE if path.matched(paths::ID_HARVEST_COMPLETE) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                                let result = api_impl.delete_completed_harvest_task(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteCompletedHarvestTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_COMPLETED_HARVEST_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteCompletedHarvestTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteCompletedHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteCompletedHarvestTaskResponse::MethodNotAllowed
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(405).expect("Unable to turn 405 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteOneCompletedHarvestTask - DELETE /harvest/complete/{HarvestTaskId}
            &hyper::Method::DELETE if path.matched(paths::ID_HARVEST_COMPLETE_HARVESTTASKID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_HARVEST_COMPLETE_HARVESTTASKID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE HARVEST_COMPLETE_HARVESTTASKID in set but failed match against \"{}\"", path, paths::REGEX_HARVEST_COMPLETE_HARVESTTASKID.as_str())
                    );

                let param_harvest_task_id = match percent_encoding::percent_decode(path_params["HarvestTaskId"].as_bytes()).decode_utf8() {
                    Ok(param_harvest_task_id) => match param_harvest_task_id.parse::<i64>() {
                        Ok(param_harvest_task_id) => param_harvest_task_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter HarvestTaskId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["HarvestTaskId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_completed_harvest_task(
                                            param_harvest_task_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOneCompletedHarvestTaskResponse::SuccessfullyDeletedCompletedHarvest
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOneCompletedHarvestTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_COMPLETED_HARVEST_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneCompletedHarvestTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOneCompletedHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOneCompletedHarvestTaskResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOneCompletedHarvestTaskResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_COMPLETED_HARVEST_TASK_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteOneScheduledHarvestTask - DELETE /harvest/task/{HarvestTaskId}
            &hyper::Method::DELETE if path.matched(paths::ID_HARVEST_TASK_HARVESTTASKID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_HARVEST_TASK_HARVESTTASKID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE HARVEST_TASK_HARVESTTASKID in set but failed match against \"{}\"", path, paths::REGEX_HARVEST_TASK_HARVESTTASKID.as_str())
                    );

                let param_harvest_task_id = match percent_encoding::percent_decode(path_params["HarvestTaskId"].as_bytes()).decode_utf8() {
                    Ok(param_harvest_task_id) => match param_harvest_task_id.parse::<i64>() {
                        Ok(param_harvest_task_id) => param_harvest_task_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter HarvestTaskId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["HarvestTaskId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_scheduled_harvest_task(
                                            param_harvest_task_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOneScheduledHarvestTaskResponse::SuccessfullyDeletedScheduledHarvestTask
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOneScheduledHarvestTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_SCHEDULED_HARVEST_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneScheduledHarvestTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOneScheduledHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOneScheduledHarvestTaskResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOneScheduledHarvestTaskResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_SCHEDULED_HARVEST_TASK_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteScheduledHarvestTask - DELETE /harvest/task
            &hyper::Method::DELETE if path.matched(paths::ID_HARVEST_TASK) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                                let result = api_impl.delete_scheduled_harvest_task(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteScheduledHarvestTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_SCHEDULED_HARVEST_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteScheduledHarvestTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteScheduledHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteScheduledHarvestTaskResponse::MethodNotAllowed
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(405).expect("Unable to turn 405 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAllCompletedHarvestTasks - GET /harvest/complete
            &hyper::Method::GET if path.matched(paths::ID_HARVEST_COMPLETE) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_box2d = query_params.iter().filter(|e| e.0 == "box2d").map(|e| e.1.to_owned())
                    .filter_map(|param_box2d| param_box2d.parse().ok())
                    .collect::<Vec<_>>();
                let param_box2d = if !param_box2d.is_empty() {
                    Some(param_box2d)
                } else {
                    None
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_all_completed_harvest_tasks(
                                            param_box2d.as_ref(),
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllCompletedHarvestTasksResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_COMPLETED_HARVEST_TASKS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllCompletedHarvestTasksResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_COMPLETED_HARVEST_TASKS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllCompletedHarvestTasksResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAllCompletedHarvestTasksResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAllCompletedHarvestTasksResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_COMPLETED_HARVEST_TASKS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAllScheduledHarvestTasks - GET /harvest/task
            &hyper::Method::GET if path.matched(paths::ID_HARVEST_TASK) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_box2d = query_params.iter().filter(|e| e.0 == "box2d").map(|e| e.1.to_owned())
                    .filter_map(|param_box2d| param_box2d.parse().ok())
                    .collect::<Vec<_>>();
                let param_box2d = if !param_box2d.is_empty() {
                    Some(param_box2d)
                } else {
                    None
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_all_scheduled_harvest_tasks(
                                            param_box2d.as_ref(),
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllScheduledHarvestTasksResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_SCHEDULED_HARVEST_TASKS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllScheduledHarvestTasksResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_SCHEDULED_HARVEST_TASKS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllScheduledHarvestTasksResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAllScheduledHarvestTasksResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAllScheduledHarvestTasksResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_SCHEDULED_HARVEST_TASKS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOneCompletedHarvestTask - GET /harvest/complete/{HarvestTaskId}
            &hyper::Method::GET if path.matched(paths::ID_HARVEST_COMPLETE_HARVESTTASKID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_HARVEST_COMPLETE_HARVESTTASKID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE HARVEST_COMPLETE_HARVESTTASKID in set but failed match against \"{}\"", path, paths::REGEX_HARVEST_COMPLETE_HARVESTTASKID.as_str())
                    );

                let param_harvest_task_id = match percent_encoding::percent_decode(path_params["HarvestTaskId"].as_bytes()).decode_utf8() {
                    Ok(param_harvest_task_id) => match param_harvest_task_id.parse::<i64>() {
                        Ok(param_harvest_task_id) => param_harvest_task_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter HarvestTaskId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["HarvestTaskId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_one_completed_harvest_task(
                                            param_harvest_task_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOneCompletedHarvestTaskResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_COMPLETED_HARVEST_TASK_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneCompletedHarvestTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_COMPLETED_HARVEST_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneCompletedHarvestTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetOneCompletedHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetOneCompletedHarvestTaskResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetOneCompletedHarvestTaskResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_COMPLETED_HARVEST_TASK_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOneScheduledHarvestTask - GET /harvest/task/{HarvestTaskId}
            &hyper::Method::GET if path.matched(paths::ID_HARVEST_TASK_HARVESTTASKID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_HARVEST_TASK_HARVESTTASKID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE HARVEST_TASK_HARVESTTASKID in set but failed match against \"{}\"", path, paths::REGEX_HARVEST_TASK_HARVESTTASKID.as_str())
                    );

                let param_harvest_task_id = match percent_encoding::percent_decode(path_params["HarvestTaskId"].as_bytes()).decode_utf8() {
                    Ok(param_harvest_task_id) => match param_harvest_task_id.parse::<i64>() {
                        Ok(param_harvest_task_id) => param_harvest_task_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter HarvestTaskId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["HarvestTaskId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_one_scheduled_harvest_task(
                                            param_harvest_task_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOneScheduledHarvestTaskResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_SCHEDULED_HARVEST_TASK_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneScheduledHarvestTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_SCHEDULED_HARVEST_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneScheduledHarvestTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetOneScheduledHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetOneScheduledHarvestTaskResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetOneScheduledHarvestTaskResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_SCHEDULED_HARVEST_TASK_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PostCompletedHarvestTask - POST /harvest/complete
            &hyper::Method::POST if path.matched(paths::ID_HARVEST_COMPLETE) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_harvest_or_task: Option<models::HarvestOrTask> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_harvest_or_task) => param_harvest_or_task,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter HarvestOrTask - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter HarvestOrTask due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_harvest_or_task = match param_harvest_or_task {
                                    Some(param_harvest_or_task) => param_harvest_or_task,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter HarvestOrTask"))
                                                        .expect("Unable to create Bad Request response for missing body parameter HarvestOrTask")),
                                };

                                let result = api_impl.post_completed_harvest_task(
                                            param_harvest_or_task,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostCompletedHarvestTaskResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_COMPLETED_HARVEST_TASK_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostCompletedHarvestTaskResponse::SeeOther
                                                    {
                                                        location
                                                    }
                                                => {
                                                    if let Some(location) = location {
                                                    let location = match header::IntoHeaderValue(location).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling location header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("location"),
                                                        location
                                                    );
                                                    }
                                                    *response.status_mut() = StatusCode::from_u16(303).expect("Unable to turn 303 into a StatusCode");
                                                },
                                                PostCompletedHarvestTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_COMPLETED_HARVEST_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostCompletedHarvestTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostCompletedHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostCompletedHarvestTaskResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_COMPLETED_HARVEST_TASK_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter HarvestOrTask: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter HarvestOrTask")),
                        }
            },

            // PostScheduledHarvestTask - POST /harvest/task
            &hyper::Method::POST if path.matched(paths::ID_HARVEST_TASK) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_harvest_task: Option<models::HarvestTask> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_harvest_task) => param_harvest_task,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter HarvestTask - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter HarvestTask due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_harvest_task = match param_harvest_task {
                                    Some(param_harvest_task) => param_harvest_task,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter HarvestTask"))
                                                        .expect("Unable to create Bad Request response for missing body parameter HarvestTask")),
                                };

                                let result = api_impl.post_scheduled_harvest_task(
                                            param_harvest_task,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostScheduledHarvestTaskResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SCHEDULED_HARVEST_TASK_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostScheduledHarvestTaskResponse::SeeOther
                                                    {
                                                        location
                                                    }
                                                => {
                                                    if let Some(location) = location {
                                                    let location = match header::IntoHeaderValue(location).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling location header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("location"),
                                                        location
                                                    );
                                                    }
                                                    *response.status_mut() = StatusCode::from_u16(303).expect("Unable to turn 303 into a StatusCode");
                                                },
                                                PostScheduledHarvestTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SCHEDULED_HARVEST_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostScheduledHarvestTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostScheduledHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostScheduledHarvestTaskResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SCHEDULED_HARVEST_TASK_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter HarvestTask: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter HarvestTask")),
                        }
            },

            // DeleteOnePlant - DELETE /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_planter_run_id = match percent_encoding::percent_decode(path_params["PlanterRunId"].as_bytes()).decode_utf8() {
                    Ok(param_planter_run_id) => match param_planter_run_id.parse::<i64>() {
                        Ok(param_planter_run_id) => param_planter_run_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlanterRunId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlanterRunId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_plant_id = match percent_encoding::percent_decode(path_params["PlantId"].as_bytes()).decode_utf8() {
                    Ok(param_plant_id) => match param_plant_id.parse::<i64>() {
                        Ok(param_plant_id) => param_plant_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlantId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlantId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_plant(
                                            param_field_id,
                                            param_planter_run_id,
                                            param_plant_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOnePlantResponse::SuccessfullyDeletedPlant
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOnePlantResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_PLANT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOnePlantResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOnePlantResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOnePlantResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOnePlantResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_PLANT_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeletePlant - DELETE /field/{FieldId}/planter/{PlanterRunId}/plant
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_planter_run_id = match percent_encoding::percent_decode(path_params["PlanterRunId"].as_bytes()).decode_utf8() {
                    Ok(param_planter_run_id) => match param_planter_run_id.parse::<i64>() {
                        Ok(param_planter_run_id) => param_planter_run_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlanterRunId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlanterRunId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_plant(
                                            param_field_id,
                                            param_planter_run_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeletePlantResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_PLANT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeletePlantResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeletePlantResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeletePlantResponse::MethodNotAllowed
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(405).expect("Unable to turn 405 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAllPlants - GET /field/{FieldId}/planter/{PlanterRunId}/plant
            &hyper::Method::GET if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_planter_run_id = match percent_encoding::percent_decode(path_params["PlanterRunId"].as_bytes()).decode_utf8() {
                    Ok(param_planter_run_id) => match param_planter_run_id.parse::<i64>() {
                        Ok(param_planter_run_id) => param_planter_run_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlanterRunId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlanterRunId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };
                let param_box2d = query_params.iter().filter(|e| e.0 == "box2d").map(|e| e.1.to_owned())
                    .filter_map(|param_box2d| param_box2d.parse().ok())
                    .collect::<Vec<_>>();
                let param_box2d = if !param_box2d.is_empty() {
                    Some(param_box2d)
                } else {
                    None
                };

                                let result = api_impl.get_all_plants(
                                            param_field_id,
                                            param_planter_run_id,
                                            param_offset,
                                            param_limit,
                                            param_box2d.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllPlantsResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_PLANTS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllPlantsResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_PLANTS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllPlantsResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAllPlantsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAllPlantsResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_PLANTS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOnePlant - GET /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}
            &hyper::Method::GET if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_planter_run_id = match percent_encoding::percent_decode(path_params["PlanterRunId"].as_bytes()).decode_utf8() {
                    Ok(param_planter_run_id) => match param_planter_run_id.parse::<i64>() {
                        Ok(param_planter_run_id) => param_planter_run_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlanterRunId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlanterRunId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_plant_id = match percent_encoding::percent_decode(path_params["PlantId"].as_bytes()).decode_utf8() {
                    Ok(param_plant_id) => match param_plant_id.parse::<i64>() {
                        Ok(param_plant_id) => param_plant_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlantId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlantId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_one_plant(
                                            param_field_id,
                                            param_planter_run_id,
                                            param_plant_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOnePlantResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_PLANT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOnePlantResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_PLANT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOnePlantResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetOnePlantResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetOnePlantResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetOnePlantResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_PLANT_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PatchOnePlant - PATCH /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}
            &hyper::Method::PATCH if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_planter_run_id = match percent_encoding::percent_decode(path_params["PlanterRunId"].as_bytes()).decode_utf8() {
                    Ok(param_planter_run_id) => match param_planter_run_id.parse::<i64>() {
                        Ok(param_planter_run_id) => param_planter_run_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlanterRunId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlanterRunId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_plant_id = match percent_encoding::percent_decode(path_params["PlantId"].as_bytes()).decode_utf8() {
                    Ok(param_plant_id) => match param_plant_id.parse::<i64>() {
                        Ok(param_plant_id) => param_plant_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlantId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlantId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_plant_patch: Option<models::PlantPatch> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_plant_patch) => param_plant_patch,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter PlantPatch - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter PlantPatch due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_plant_patch = match param_plant_patch {
                                    Some(param_plant_patch) => param_plant_patch,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter PlantPatch"))
                                                        .expect("Unable to create Bad Request response for missing body parameter PlantPatch")),
                                };

                                let result = api_impl.patch_one_plant(
                                            param_field_id,
                                            param_planter_run_id,
                                            param_plant_id,
                                            param_plant_patch,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PatchOnePlantResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_PLANT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PatchOnePlantResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_PLANT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PatchOnePlantResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PatchOnePlantResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PatchOnePlantResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                PatchOnePlantResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_PLANT_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter PlantPatch: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter PlantPatch")),
                        }
            },

            // PostPlant - POST /field/{FieldId}/planter/{PlanterRunId}/plant
            &hyper::Method::POST if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_planter_run_id = match percent_encoding::percent_decode(path_params["PlanterRunId"].as_bytes()).decode_utf8() {
                    Ok(param_planter_run_id) => match param_planter_run_id.parse::<i64>() {
                        Ok(param_planter_run_id) => param_planter_run_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlanterRunId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlanterRunId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_plant: Option<models::Plant> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_plant) => param_plant,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter Plant - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter Plant due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_plant = match param_plant {
                                    Some(param_plant) => param_plant,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter Plant"))
                                                        .expect("Unable to create Bad Request response for missing body parameter Plant")),
                                };

                                let result = api_impl.post_plant(
                                            param_field_id,
                                            param_planter_run_id,
                                            param_plant,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostPlantResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_PLANT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostPlantResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_PLANT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostPlantResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostPlantResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostPlantResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_PLANT_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Plant: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Plant")),
                        }
            },

            // DeleteOnePlanterRun - DELETE /field/{FieldId}/planter/{PlanterRunId}
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER_PLANTERRUNID in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_planter_run_id = match percent_encoding::percent_decode(path_params["PlanterRunId"].as_bytes()).decode_utf8() {
                    Ok(param_planter_run_id) => match param_planter_run_id.parse::<i64>() {
                        Ok(param_planter_run_id) => param_planter_run_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlanterRunId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlanterRunId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_planter_run(
                                            param_field_id,
                                            param_planter_run_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOnePlanterRunResponse::SuccessfullyDeletedPlanterRun
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOnePlanterRunResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_PLANTER_RUN_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOnePlanterRunResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOnePlanterRunResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOnePlanterRunResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOnePlanterRunResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_PLANTER_RUN_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeletePlanterRun - DELETE /field/{FieldId}/planter
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD_FIELDID_PLANTER) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_planter_run(
                                            param_field_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeletePlanterRunResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_PLANTER_RUN_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeletePlanterRunResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeletePlanterRunResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeletePlanterRunResponse::MethodNotAllowed
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(405).expect("Unable to turn 405 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAllPlanterRuns - GET /field/{FieldId}/planter
            &hyper::Method::GET if path.matched(paths::ID_FIELD_FIELDID_PLANTER) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_box2d = query_params.iter().filter(|e| e.0 == "box2d").map(|e| e.1.to_owned())
                    .filter_map(|param_box2d| param_box2d.parse().ok())
                    .collect::<Vec<_>>();
                let param_box2d = if !param_box2d.is_empty() {
                    Some(param_box2d)
                } else {
                    None
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_all_planter_runs(
                                            param_field_id,
                                            param_box2d.as_ref(),
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllPlanterRunsResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_PLANTER_RUNS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllPlanterRunsResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_PLANTER_RUNS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllPlanterRunsResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAllPlanterRunsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAllPlanterRunsResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_PLANTER_RUNS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOnePlanterRun - GET /field/{FieldId}/planter/{PlanterRunId}
            &hyper::Method::GET if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER_PLANTERRUNID in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_planter_run_id = match percent_encoding::percent_decode(path_params["PlanterRunId"].as_bytes()).decode_utf8() {
                    Ok(param_planter_run_id) => match param_planter_run_id.parse::<i64>() {
                        Ok(param_planter_run_id) => param_planter_run_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlanterRunId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlanterRunId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_one_planter_run(
                                            param_field_id,
                                            param_planter_run_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOnePlanterRunResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_PLANTER_RUN_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOnePlanterRunResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_PLANTER_RUN_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOnePlanterRunResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetOnePlanterRunResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetOnePlanterRunResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetOnePlanterRunResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_PLANTER_RUN_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PostPlanterRun - POST /field/{FieldId}/planter
            &hyper::Method::POST if path.matched(paths::ID_FIELD_FIELDID_PLANTER) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_planter_run: Option<models::PlanterRun> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_planter_run) => param_planter_run,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter PlanterRun - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter PlanterRun due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_planter_run = match param_planter_run {
                                    Some(param_planter_run) => param_planter_run,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter PlanterRun"))
                                                        .expect("Unable to create Bad Request response for missing body parameter PlanterRun")),
                                };

                                let result = api_impl.post_planter_run(
                                            param_field_id,
                                            param_planter_run,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostPlanterRunResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_PLANTER_RUN_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostPlanterRunResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_PLANTER_RUN_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostPlanterRunResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostPlanterRunResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostPlanterRunResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_PLANTER_RUN_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter PlanterRun: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter PlanterRun")),
                        }
            },

            // DeleteOneQualityMeasurement - DELETE /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement/{QualityMeasurementId}
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT_QUALITYMEASUREMENTID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT_QUALITYMEASUREMENTID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT_QUALITYMEASUREMENTID in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT_QUALITYMEASUREMENTID.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_planter_run_id = match percent_encoding::percent_decode(path_params["PlanterRunId"].as_bytes()).decode_utf8() {
                    Ok(param_planter_run_id) => match param_planter_run_id.parse::<i64>() {
                        Ok(param_planter_run_id) => param_planter_run_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlanterRunId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlanterRunId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_plant_id = match percent_encoding::percent_decode(path_params["PlantId"].as_bytes()).decode_utf8() {
                    Ok(param_plant_id) => match param_plant_id.parse::<i64>() {
                        Ok(param_plant_id) => param_plant_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlantId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlantId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_quality_measurement_id = match percent_encoding::percent_decode(path_params["QualityMeasurementId"].as_bytes()).decode_utf8() {
                    Ok(param_quality_measurement_id) => match param_quality_measurement_id.parse::<i64>() {
                        Ok(param_quality_measurement_id) => param_quality_measurement_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter QualityMeasurementId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["QualityMeasurementId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_quality_measurement(
                                            param_field_id,
                                            param_planter_run_id,
                                            param_plant_id,
                                            param_quality_measurement_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOneQualityMeasurementResponse::SuccessfullyDeletedQualityMeasurement
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOneQualityMeasurementResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_QUALITY_MEASUREMENT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneQualityMeasurementResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOneQualityMeasurementResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOneQualityMeasurementResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOneQualityMeasurementResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_QUALITY_MEASUREMENT_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteQualityMeasurement - DELETE /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_planter_run_id = match percent_encoding::percent_decode(path_params["PlanterRunId"].as_bytes()).decode_utf8() {
                    Ok(param_planter_run_id) => match param_planter_run_id.parse::<i64>() {
                        Ok(param_planter_run_id) => param_planter_run_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlanterRunId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlanterRunId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_plant_id = match percent_encoding::percent_decode(path_params["PlantId"].as_bytes()).decode_utf8() {
                    Ok(param_plant_id) => match param_plant_id.parse::<i64>() {
                        Ok(param_plant_id) => param_plant_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlantId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlantId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_quality_measurement(
                                            param_field_id,
                                            param_planter_run_id,
                                            param_plant_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteQualityMeasurementResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_QUALITY_MEASUREMENT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteQualityMeasurementResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteQualityMeasurementResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteQualityMeasurementResponse::MethodNotAllowed
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(405).expect("Unable to turn 405 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAllQualityMeasurements - GET /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement
            &hyper::Method::GET if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_planter_run_id = match percent_encoding::percent_decode(path_params["PlanterRunId"].as_bytes()).decode_utf8() {
                    Ok(param_planter_run_id) => match param_planter_run_id.parse::<i64>() {
                        Ok(param_planter_run_id) => param_planter_run_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlanterRunId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlanterRunId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_plant_id = match percent_encoding::percent_decode(path_params["PlantId"].as_bytes()).decode_utf8() {
                    Ok(param_plant_id) => match param_plant_id.parse::<i64>() {
                        Ok(param_plant_id) => param_plant_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlantId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlantId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_all_quality_measurements(
                                            param_field_id,
                                            param_planter_run_id,
                                            param_plant_id,
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllQualityMeasurementsResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_QUALITY_MEASUREMENTS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllQualityMeasurementsResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_QUALITY_MEASUREMENTS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllQualityMeasurementsResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAllQualityMeasurementsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAllQualityMeasurementsResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_QUALITY_MEASUREMENTS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOneQualityMeasurement - GET /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement/{QualityMeasurementId}
            &hyper::Method::GET if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT_QUALITYMEASUREMENTID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT_QUALITYMEASUREMENTID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT_QUALITYMEASUREMENTID in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT_QUALITYMEASUREMENTID.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_planter_run_id = match percent_encoding::percent_decode(path_params["PlanterRunId"].as_bytes()).decode_utf8() {
                    Ok(param_planter_run_id) => match param_planter_run_id.parse::<i64>() {
                        Ok(param_planter_run_id) => param_planter_run_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlanterRunId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlanterRunId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_plant_id = match percent_encoding::percent_decode(path_params["PlantId"].as_bytes()).decode_utf8() {
                    Ok(param_plant_id) => match param_plant_id.parse::<i64>() {
                        Ok(param_plant_id) => param_plant_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlantId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlantId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_quality_measurement_id = match percent_encoding::percent_decode(path_params["QualityMeasurementId"].as_bytes()).decode_utf8() {
                    Ok(param_quality_measurement_id) => match param_quality_measurement_id.parse::<i64>() {
                        Ok(param_quality_measurement_id) => param_quality_measurement_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter QualityMeasurementId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["QualityMeasurementId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_one_quality_measurement(
                                            param_field_id,
                                            param_planter_run_id,
                                            param_plant_id,
                                            param_quality_measurement_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOneQualityMeasurementResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_QUALITY_MEASUREMENT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneQualityMeasurementResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_QUALITY_MEASUREMENT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneQualityMeasurementResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetOneQualityMeasurementResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetOneQualityMeasurementResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetOneQualityMeasurementResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_QUALITY_MEASUREMENT_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PostQualityMeasurement - POST /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement
            &hyper::Method::POST if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT in set but failed match against \"{}\"", path, paths::REGEX_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT.as_str())
                    );

                let param_field_id = match percent_encoding::percent_decode(path_params["FieldId"].as_bytes()).decode_utf8() {
                    Ok(param_field_id) => match param_field_id.parse::<i64>() {
                        Ok(param_field_id) => param_field_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter FieldId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["FieldId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_planter_run_id = match percent_encoding::percent_decode(path_params["PlanterRunId"].as_bytes()).decode_utf8() {
                    Ok(param_planter_run_id) => match param_planter_run_id.parse::<i64>() {
                        Ok(param_planter_run_id) => param_planter_run_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlanterRunId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlanterRunId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_plant_id = match percent_encoding::percent_decode(path_params["PlantId"].as_bytes()).decode_utf8() {
                    Ok(param_plant_id) => match param_plant_id.parse::<i64>() {
                        Ok(param_plant_id) => param_plant_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter PlantId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["PlantId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_quality_measurement: Option<models::QualityMeasurement> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_quality_measurement) => param_quality_measurement,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter QualityMeasurement - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter QualityMeasurement due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_quality_measurement = match param_quality_measurement {
                                    Some(param_quality_measurement) => param_quality_measurement,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter QualityMeasurement"))
                                                        .expect("Unable to create Bad Request response for missing body parameter QualityMeasurement")),
                                };

                                let result = api_impl.post_quality_measurement(
                                            param_field_id,
                                            param_planter_run_id,
                                            param_plant_id,
                                            param_quality_measurement,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostQualityMeasurementResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_QUALITY_MEASUREMENT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostQualityMeasurementResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_QUALITY_MEASUREMENT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostQualityMeasurementResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostQualityMeasurementResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostQualityMeasurementResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_QUALITY_MEASUREMENT_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter QualityMeasurement: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter QualityMeasurement")),
                        }
            },

            // DeleteCompletedSprayTask - DELETE /spray/complete
            &hyper::Method::DELETE if path.matched(paths::ID_SPRAY_COMPLETE) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                                let result = api_impl.delete_completed_spray_task(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteCompletedSprayTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_COMPLETED_SPRAY_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteCompletedSprayTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteCompletedSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteCompletedSprayTaskResponse::MethodNotAllowed
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(405).expect("Unable to turn 405 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteOneCompletedSprayTask - DELETE /spray/complete/{SprayTaskId}
            &hyper::Method::DELETE if path.matched(paths::ID_SPRAY_COMPLETE_SPRAYTASKID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SPRAY_COMPLETE_SPRAYTASKID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SPRAY_COMPLETE_SPRAYTASKID in set but failed match against \"{}\"", path, paths::REGEX_SPRAY_COMPLETE_SPRAYTASKID.as_str())
                    );

                let param_spray_task_id = match percent_encoding::percent_decode(path_params["SprayTaskId"].as_bytes()).decode_utf8() {
                    Ok(param_spray_task_id) => match param_spray_task_id.parse::<i64>() {
                        Ok(param_spray_task_id) => param_spray_task_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SprayTaskId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SprayTaskId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_completed_spray_task(
                                            param_spray_task_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOneCompletedSprayTaskResponse::SuccessfullyDeletedCompletedSpray
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOneCompletedSprayTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_COMPLETED_SPRAY_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneCompletedSprayTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOneCompletedSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOneCompletedSprayTaskResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOneCompletedSprayTaskResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_COMPLETED_SPRAY_TASK_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteOneScheduledSprayTask - DELETE /spray/task/{SprayTaskId}
            &hyper::Method::DELETE if path.matched(paths::ID_SPRAY_TASK_SPRAYTASKID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SPRAY_TASK_SPRAYTASKID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SPRAY_TASK_SPRAYTASKID in set but failed match against \"{}\"", path, paths::REGEX_SPRAY_TASK_SPRAYTASKID.as_str())
                    );

                let param_spray_task_id = match percent_encoding::percent_decode(path_params["SprayTaskId"].as_bytes()).decode_utf8() {
                    Ok(param_spray_task_id) => match param_spray_task_id.parse::<i64>() {
                        Ok(param_spray_task_id) => param_spray_task_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SprayTaskId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SprayTaskId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_scheduled_spray_task(
                                            param_spray_task_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOneScheduledSprayTaskResponse::SuccessfullyDeletedScheduledSprayTask
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOneScheduledSprayTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_SCHEDULED_SPRAY_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneScheduledSprayTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOneScheduledSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOneScheduledSprayTaskResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOneScheduledSprayTaskResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_SCHEDULED_SPRAY_TASK_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteScheduledSprayTask - DELETE /spray/task
            &hyper::Method::DELETE if path.matched(paths::ID_SPRAY_TASK) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                                let result = api_impl.delete_scheduled_spray_task(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteScheduledSprayTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_SCHEDULED_SPRAY_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteScheduledSprayTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteScheduledSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteScheduledSprayTaskResponse::MethodNotAllowed
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(405).expect("Unable to turn 405 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAllCompletedSprayTasks - GET /spray/complete
            &hyper::Method::GET if path.matched(paths::ID_SPRAY_COMPLETE) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_box2d = query_params.iter().filter(|e| e.0 == "box2d").map(|e| e.1.to_owned())
                    .filter_map(|param_box2d| param_box2d.parse().ok())
                    .collect::<Vec<_>>();
                let param_box2d = if !param_box2d.is_empty() {
                    Some(param_box2d)
                } else {
                    None
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_all_completed_spray_tasks(
                                            param_box2d.as_ref(),
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllCompletedSprayTasksResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_COMPLETED_SPRAY_TASKS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllCompletedSprayTasksResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_COMPLETED_SPRAY_TASKS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllCompletedSprayTasksResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAllCompletedSprayTasksResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAllCompletedSprayTasksResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_COMPLETED_SPRAY_TASKS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAllScheduledSprayTasks - GET /spray/task
            &hyper::Method::GET if path.matched(paths::ID_SPRAY_TASK) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_box2d = query_params.iter().filter(|e| e.0 == "box2d").map(|e| e.1.to_owned())
                    .filter_map(|param_box2d| param_box2d.parse().ok())
                    .collect::<Vec<_>>();
                let param_box2d = if !param_box2d.is_empty() {
                    Some(param_box2d)
                } else {
                    None
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_all_scheduled_spray_tasks(
                                            param_box2d.as_ref(),
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllScheduledSprayTasksResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_SCHEDULED_SPRAY_TASKS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllScheduledSprayTasksResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_SCHEDULED_SPRAY_TASKS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllScheduledSprayTasksResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAllScheduledSprayTasksResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAllScheduledSprayTasksResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_SCHEDULED_SPRAY_TASKS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOneCompletedSprayTask - GET /spray/complete/{SprayTaskId}
            &hyper::Method::GET if path.matched(paths::ID_SPRAY_COMPLETE_SPRAYTASKID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SPRAY_COMPLETE_SPRAYTASKID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SPRAY_COMPLETE_SPRAYTASKID in set but failed match against \"{}\"", path, paths::REGEX_SPRAY_COMPLETE_SPRAYTASKID.as_str())
                    );

                let param_spray_task_id = match percent_encoding::percent_decode(path_params["SprayTaskId"].as_bytes()).decode_utf8() {
                    Ok(param_spray_task_id) => match param_spray_task_id.parse::<i64>() {
                        Ok(param_spray_task_id) => param_spray_task_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SprayTaskId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SprayTaskId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_one_completed_spray_task(
                                            param_spray_task_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOneCompletedSprayTaskResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_COMPLETED_SPRAY_TASK_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneCompletedSprayTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_COMPLETED_SPRAY_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneCompletedSprayTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetOneCompletedSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetOneCompletedSprayTaskResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetOneCompletedSprayTaskResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_COMPLETED_SPRAY_TASK_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOneScheduledSprayTask - GET /spray/task/{SprayTaskId}
            &hyper::Method::GET if path.matched(paths::ID_SPRAY_TASK_SPRAYTASKID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SPRAY_TASK_SPRAYTASKID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SPRAY_TASK_SPRAYTASKID in set but failed match against \"{}\"", path, paths::REGEX_SPRAY_TASK_SPRAYTASKID.as_str())
                    );

                let param_spray_task_id = match percent_encoding::percent_decode(path_params["SprayTaskId"].as_bytes()).decode_utf8() {
                    Ok(param_spray_task_id) => match param_spray_task_id.parse::<i64>() {
                        Ok(param_spray_task_id) => param_spray_task_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SprayTaskId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SprayTaskId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_one_scheduled_spray_task(
                                            param_spray_task_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOneScheduledSprayTaskResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_SCHEDULED_SPRAY_TASK_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneScheduledSprayTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_SCHEDULED_SPRAY_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneScheduledSprayTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetOneScheduledSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetOneScheduledSprayTaskResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetOneScheduledSprayTaskResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_SCHEDULED_SPRAY_TASK_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PostCompletedSprayTask - POST /spray/complete
            &hyper::Method::POST if path.matched(paths::ID_SPRAY_COMPLETE) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_spray_task: Option<models::SprayTask> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_spray_task) => param_spray_task,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter SprayTask - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter SprayTask due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_spray_task = match param_spray_task {
                                    Some(param_spray_task) => param_spray_task,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter SprayTask"))
                                                        .expect("Unable to create Bad Request response for missing body parameter SprayTask")),
                                };

                                let result = api_impl.post_completed_spray_task(
                                            param_spray_task,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostCompletedSprayTaskResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_COMPLETED_SPRAY_TASK_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostCompletedSprayTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_COMPLETED_SPRAY_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostCompletedSprayTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostCompletedSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostCompletedSprayTaskResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_COMPLETED_SPRAY_TASK_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter SprayTask: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter SprayTask")),
                        }
            },

            // PostScheduledSprayTask - POST /spray/task
            &hyper::Method::POST if path.matched(paths::ID_SPRAY_TASK) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_spray_task: Option<models::SprayTask> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_spray_task) => param_spray_task,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter SprayTask - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter SprayTask due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_spray_task = match param_spray_task {
                                    Some(param_spray_task) => param_spray_task,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter SprayTask"))
                                                        .expect("Unable to create Bad Request response for missing body parameter SprayTask")),
                                };

                                let result = api_impl.post_scheduled_spray_task(
                                            param_spray_task,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostScheduledSprayTaskResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SCHEDULED_SPRAY_TASK_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostScheduledSprayTaskResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SCHEDULED_SPRAY_TASK_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostScheduledSprayTaskResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostScheduledSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostScheduledSprayTaskResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_SCHEDULED_SPRAY_TASK_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter SprayTask: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter SprayTask")),
                        }
            },

            // DeleteOneTerrain - DELETE /terrain/{TerrainId}
            &hyper::Method::DELETE if path.matched(paths::ID_TERRAIN_TERRAINID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_TERRAIN_TERRAINID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE TERRAIN_TERRAINID in set but failed match against \"{}\"", path, paths::REGEX_TERRAIN_TERRAINID.as_str())
                    );

                let param_terrain_id = match percent_encoding::percent_decode(path_params["TerrainId"].as_bytes()).decode_utf8() {
                    Ok(param_terrain_id) => match param_terrain_id.parse::<i64>() {
                        Ok(param_terrain_id) => param_terrain_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter TerrainId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["TerrainId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_terrain(
                                            param_terrain_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOneTerrainResponse::SuccessfullyDeletedTerrainModel
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOneTerrainResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_TERRAIN_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneTerrainResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOneTerrainResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOneTerrainResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOneTerrainResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_TERRAIN_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteTerrain - DELETE /terrain
            &hyper::Method::DELETE if path.matched(paths::ID_TERRAIN) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                                let result = api_impl.delete_terrain(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteTerrainResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_TERRAIN_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteTerrainResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteTerrainResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteTerrainResponse::MethodNotAllowed
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(405).expect("Unable to turn 405 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAllTerrains - GET /terrain
            &hyper::Method::GET if path.matched(paths::ID_TERRAIN) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_all_terrains(
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllTerrainsResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_TERRAINS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllTerrainsResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_TERRAINS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllTerrainsResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAllTerrainsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAllTerrainsResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_TERRAINS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOneTerrain - GET /terrain/{TerrainId}
            &hyper::Method::GET if path.matched(paths::ID_TERRAIN_TERRAINID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_TERRAIN_TERRAINID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE TERRAIN_TERRAINID in set but failed match against \"{}\"", path, paths::REGEX_TERRAIN_TERRAINID.as_str())
                    );

                let param_terrain_id = match percent_encoding::percent_decode(path_params["TerrainId"].as_bytes()).decode_utf8() {
                    Ok(param_terrain_id) => match param_terrain_id.parse::<i64>() {
                        Ok(param_terrain_id) => param_terrain_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter TerrainId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["TerrainId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_one_terrain(
                                            param_terrain_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOneTerrainResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_TERRAIN_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneTerrainResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_TERRAIN_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneTerrainResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetOneTerrainResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetOneTerrainResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetOneTerrainResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_TERRAIN_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteOneAccessRight - DELETE /token/{UserId}/{Tokenstr}/access_right/{AccessRightPath}
            &hyper::Method::DELETE if path.matched(paths::ID_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT_ACCESSRIGHTPATH) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT_ACCESSRIGHTPATH
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE TOKEN_USERID_TOKENSTR_ACCESS_RIGHT_ACCESSRIGHTPATH in set but failed match against \"{}\"", path, paths::REGEX_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT_ACCESSRIGHTPATH.as_str())
                    );

                let param_user_id = match percent_encoding::percent_decode(path_params["UserId"].as_bytes()).decode_utf8() {
                    Ok(param_user_id) => match param_user_id.parse::<i32>() {
                        Ok(param_user_id) => param_user_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter UserId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["UserId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_tokenstr = match percent_encoding::percent_decode(path_params["Tokenstr"].as_bytes()).decode_utf8() {
                    Ok(param_tokenstr) => match param_tokenstr.parse::<String>() {
                        Ok(param_tokenstr) => param_tokenstr,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter Tokenstr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["Tokenstr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_access_right_path = match percent_encoding::percent_decode(path_params["AccessRightPath"].as_bytes()).decode_utf8() {
                    Ok(param_access_right_path) => match param_access_right_path.parse::<String>() {
                        Ok(param_access_right_path) => param_access_right_path,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter AccessRightPath: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["AccessRightPath"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_access_right(
                                            param_user_id,
                                            param_tokenstr,
                                            param_access_right_path,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOneAccessRightResponse::SuccessfullyDeletedAccessRight
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOneAccessRightResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_ACCESS_RIGHT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneAccessRightResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOneAccessRightResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOneAccessRightResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOneAccessRightResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_ACCESS_RIGHT_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteOneToken - DELETE /token/{UserId}/{Tokenstr}
            &hyper::Method::DELETE if path.matched(paths::ID_TOKEN_USERID_TOKENSTR) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_TOKEN_USERID_TOKENSTR
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE TOKEN_USERID_TOKENSTR in set but failed match against \"{}\"", path, paths::REGEX_TOKEN_USERID_TOKENSTR.as_str())
                    );

                let param_user_id = match percent_encoding::percent_decode(path_params["UserId"].as_bytes()).decode_utf8() {
                    Ok(param_user_id) => match param_user_id.parse::<i32>() {
                        Ok(param_user_id) => param_user_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter UserId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["UserId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_tokenstr = match percent_encoding::percent_decode(path_params["Tokenstr"].as_bytes()).decode_utf8() {
                    Ok(param_tokenstr) => match param_tokenstr.parse::<String>() {
                        Ok(param_tokenstr) => param_tokenstr,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter Tokenstr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["Tokenstr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_token(
                                            param_user_id,
                                            param_tokenstr,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOneTokenResponse::SuccessfullyDeletedToken
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOneTokenResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_TOKEN_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneTokenResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOneTokenResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOneTokenResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOneTokenResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_TOKEN_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteOneUser - DELETE /token/{UserId}
            &hyper::Method::DELETE if path.matched(paths::ID_TOKEN_USERID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_TOKEN_USERID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE TOKEN_USERID in set but failed match against \"{}\"", path, paths::REGEX_TOKEN_USERID.as_str())
                    );

                let param_user_id = match percent_encoding::percent_decode(path_params["UserId"].as_bytes()).decode_utf8() {
                    Ok(param_user_id) => match param_user_id.parse::<i32>() {
                        Ok(param_user_id) => param_user_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter UserId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["UserId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_user(
                                            param_user_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOneUserResponse::SuccessfullyDeletedUser
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_USER_SUCCESSFULLY_DELETED_USER"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneUserResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_USER_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneUserResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOneUserResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOneUserResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOneUserResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_USER_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAccessRights - GET /token/{UserId}/{Tokenstr}/access_right
            &hyper::Method::GET if path.matched(paths::ID_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE TOKEN_USERID_TOKENSTR_ACCESS_RIGHT in set but failed match against \"{}\"", path, paths::REGEX_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT.as_str())
                    );

                let param_user_id = match percent_encoding::percent_decode(path_params["UserId"].as_bytes()).decode_utf8() {
                    Ok(param_user_id) => match param_user_id.parse::<i32>() {
                        Ok(param_user_id) => param_user_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter UserId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["UserId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_tokenstr = match percent_encoding::percent_decode(path_params["Tokenstr"].as_bytes()).decode_utf8() {
                    Ok(param_tokenstr) => match param_tokenstr.parse::<String>() {
                        Ok(param_tokenstr) => param_tokenstr,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter Tokenstr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["Tokenstr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_access_rights(
                                            param_user_id,
                                            param_tokenstr,
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAccessRightsResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ACCESS_RIGHTS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAccessRightsResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ACCESS_RIGHTS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAccessRightsResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAccessRightsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAccessRightsResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetAccessRightsResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ACCESS_RIGHTS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAllTokens - GET /token
            &hyper::Method::GET if path.matched(paths::ID_TOKEN) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_tokenstr_contains = query_params.iter().filter(|e| e.0 == "tokenstrContains").map(|e| e.1.to_owned())
                    .filter_map(|param_tokenstr_contains| param_tokenstr_contains.parse().ok())
                    .collect::<Vec<_>>();
                let param_tokenstr_contains = if !param_tokenstr_contains.is_empty() {
                    Some(param_tokenstr_contains)
                } else {
                    None
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_all_tokens(
                                            param_tokenstr_contains.as_ref(),
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllTokensResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_TOKENS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllTokensResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_TOKENS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllTokensResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAllTokensResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAllTokensResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_TOKENS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOneAccessRight - GET /token/{UserId}/{Tokenstr}/access_right/{AccessRightPath}
            &hyper::Method::GET if path.matched(paths::ID_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT_ACCESSRIGHTPATH) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT_ACCESSRIGHTPATH
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE TOKEN_USERID_TOKENSTR_ACCESS_RIGHT_ACCESSRIGHTPATH in set but failed match against \"{}\"", path, paths::REGEX_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT_ACCESSRIGHTPATH.as_str())
                    );

                let param_user_id = match percent_encoding::percent_decode(path_params["UserId"].as_bytes()).decode_utf8() {
                    Ok(param_user_id) => match param_user_id.parse::<i32>() {
                        Ok(param_user_id) => param_user_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter UserId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["UserId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_tokenstr = match percent_encoding::percent_decode(path_params["Tokenstr"].as_bytes()).decode_utf8() {
                    Ok(param_tokenstr) => match param_tokenstr.parse::<String>() {
                        Ok(param_tokenstr) => param_tokenstr,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter Tokenstr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["Tokenstr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_access_right_path = match percent_encoding::percent_decode(path_params["AccessRightPath"].as_bytes()).decode_utf8() {
                    Ok(param_access_right_path) => match param_access_right_path.parse::<String>() {
                        Ok(param_access_right_path) => param_access_right_path,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter AccessRightPath: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["AccessRightPath"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_one_access_right(
                                            param_user_id,
                                            param_tokenstr,
                                            param_access_right_path,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOneAccessRightResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_ACCESS_RIGHT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneAccessRightResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_ACCESS_RIGHT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneAccessRightResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetOneAccessRightResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetOneAccessRightResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetOneAccessRightResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_ACCESS_RIGHT_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetUserTokens - GET /token/{UserId}
            &hyper::Method::GET if path.matched(paths::ID_TOKEN_USERID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_TOKEN_USERID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE TOKEN_USERID in set but failed match against \"{}\"", path, paths::REGEX_TOKEN_USERID.as_str())
                    );

                let param_user_id = match percent_encoding::percent_decode(path_params["UserId"].as_bytes()).decode_utf8() {
                    Ok(param_user_id) => match param_user_id.parse::<i32>() {
                        Ok(param_user_id) => param_user_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter UserId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["UserId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_tokenstr_contains = query_params.iter().filter(|e| e.0 == "tokenstrContains").map(|e| e.1.to_owned())
                    .filter_map(|param_tokenstr_contains| param_tokenstr_contains.parse().ok())
                    .collect::<Vec<_>>();
                let param_tokenstr_contains = if !param_tokenstr_contains.is_empty() {
                    Some(param_tokenstr_contains)
                } else {
                    None
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_user_tokens(
                                            param_user_id,
                                            param_tokenstr_contains.as_ref(),
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetUserTokensResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_USER_TOKENS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetUserTokensResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_USER_TOKENS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetUserTokensResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetUserTokensResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetUserTokensResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetUserTokensResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_USER_TOKENS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PostAccessRight - POST /token/{UserId}/{Tokenstr}/access_right
            &hyper::Method::POST if path.matched(paths::ID_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE TOKEN_USERID_TOKENSTR_ACCESS_RIGHT in set but failed match against \"{}\"", path, paths::REGEX_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT.as_str())
                    );

                let param_user_id = match percent_encoding::percent_decode(path_params["UserId"].as_bytes()).decode_utf8() {
                    Ok(param_user_id) => match param_user_id.parse::<i32>() {
                        Ok(param_user_id) => param_user_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter UserId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["UserId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_tokenstr = match percent_encoding::percent_decode(path_params["Tokenstr"].as_bytes()).decode_utf8() {
                    Ok(param_tokenstr) => match param_tokenstr.parse::<String>() {
                        Ok(param_tokenstr) => param_tokenstr,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter Tokenstr: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["Tokenstr"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_access_right: Option<models::AccessRight> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_access_right) => param_access_right,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter AccessRight - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter AccessRight due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_access_right = match param_access_right {
                                    Some(param_access_right) => param_access_right,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter AccessRight"))
                                                        .expect("Unable to create Bad Request response for missing body parameter AccessRight")),
                                };

                                let result = api_impl.post_access_right(
                                            param_user_id,
                                            param_tokenstr,
                                            param_access_right,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostAccessRightResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_ACCESS_RIGHT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostAccessRightResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_ACCESS_RIGHT_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostAccessRightResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostAccessRightResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostAccessRightResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                PostAccessRightResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_ACCESS_RIGHT_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter AccessRight: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter AccessRight")),
                        }
            },

            // PostToken - POST /token
            &hyper::Method::POST if path.matched(paths::ID_TOKEN) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_token: Option<models::Token> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_token) => param_token,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter Token - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter Token due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_token = match param_token {
                                    Some(param_token) => param_token,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter Token"))
                                                        .expect("Unable to create Bad Request response for missing body parameter Token")),
                                };

                                let result = api_impl.post_token(
                                            param_token,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostTokenResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_TOKEN_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostTokenResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_TOKEN_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostTokenResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostTokenResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostTokenResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_TOKEN_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Token: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Token")),
                        }
            },

            // PostUserToken - POST /token/{UserId}
            &hyper::Method::POST if path.matched(paths::ID_TOKEN_USERID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_TOKEN_USERID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE TOKEN_USERID in set but failed match against \"{}\"", path, paths::REGEX_TOKEN_USERID.as_str())
                    );

                let param_user_id = match percent_encoding::percent_decode(path_params["UserId"].as_bytes()).decode_utf8() {
                    Ok(param_user_id) => match param_user_id.parse::<i32>() {
                        Ok(param_user_id) => param_user_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter UserId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["UserId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_user_token: Option<models::UserToken> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_user_token) => param_user_token,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter UserToken - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter UserToken due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_user_token = match param_user_token {
                                    Some(param_user_token) => param_user_token,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter UserToken"))
                                                        .expect("Unable to create Bad Request response for missing body parameter UserToken")),
                                };

                                let result = api_impl.post_user_token(
                                            param_user_id,
                                            param_user_token,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostUserTokenResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_USER_TOKEN_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostUserTokenResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_USER_TOKEN_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostUserTokenResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostUserTokenResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostUserTokenResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_USER_TOKEN_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter UserToken: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter UserToken")),
                        }
            },

            // DeleteOneWeatherData - DELETE /weather/data/{WeatherStationId}/{MeasurementDate}
            &hyper::Method::DELETE if path.matched(paths::ID_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE in set but failed match against \"{}\"", path, paths::REGEX_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE.as_str())
                    );

                let param_weather_station_id = match percent_encoding::percent_decode(path_params["WeatherStationId"].as_bytes()).decode_utf8() {
                    Ok(param_weather_station_id) => match param_weather_station_id.parse::<i32>() {
                        Ok(param_weather_station_id) => param_weather_station_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter WeatherStationId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["WeatherStationId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_measurement_date = match percent_encoding::percent_decode(path_params["MeasurementDate"].as_bytes()).decode_utf8() {
                    Ok(param_measurement_date) => match param_measurement_date.parse::<String>() {
                        Ok(param_measurement_date) => param_measurement_date,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter MeasurementDate: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["MeasurementDate"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_weather_data(
                                            param_weather_station_id,
                                            param_measurement_date,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOneWeatherDataResponse::SuccessfullyDeletedWeatherData
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOneWeatherDataResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_WEATHER_DATA_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneWeatherDataResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOneWeatherDataResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOneWeatherDataResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOneWeatherDataResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_WEATHER_DATA_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAllWeatherData - GET /weather/data
            &hyper::Method::GET if path.matched(paths::ID_WEATHER_DATA) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_station_ids = query_params.iter().filter(|e| e.0 == "stationIds").map(|e| e.1.to_owned())
                    .filter_map(|param_station_ids| param_station_ids.parse().ok())
                    .collect::<Vec<_>>();
                let param_station_ids = if !param_station_ids.is_empty() {
                    Some(param_station_ids)
                } else {
                    None
                };
                let param_start_date = query_params.iter().filter(|e| e.0 == "startDate").map(|e| e.1.to_owned())
                    .nth(0);
                let param_start_date = match param_start_date {
                    Some(param_start_date) => {
                        let param_start_date =
                            <String as std::str::FromStr>::from_str
                                (&param_start_date);
                        match param_start_date {
                            Ok(param_start_date) => Some(param_start_date),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter startDate - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter startDate")),
                        }
                    },
                    None => None,
                };
                let param_end_date = query_params.iter().filter(|e| e.0 == "endDate").map(|e| e.1.to_owned())
                    .nth(0);
                let param_end_date = match param_end_date {
                    Some(param_end_date) => {
                        let param_end_date =
                            <String as std::str::FromStr>::from_str
                                (&param_end_date);
                        match param_end_date {
                            Ok(param_end_date) => Some(param_end_date),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter endDate - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter endDate")),
                        }
                    },
                    None => None,
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_all_weather_data(
                                            param_station_ids.as_ref(),
                                            param_start_date,
                                            param_end_date,
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllWeatherDataResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_WEATHER_DATA_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllWeatherDataResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_WEATHER_DATA_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllWeatherDataResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAllWeatherDataResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAllWeatherDataResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_WEATHER_DATA_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOneWeatherData - GET /weather/data/{WeatherStationId}/{MeasurementDate}
            &hyper::Method::GET if path.matched(paths::ID_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE in set but failed match against \"{}\"", path, paths::REGEX_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE.as_str())
                    );

                let param_weather_station_id = match percent_encoding::percent_decode(path_params["WeatherStationId"].as_bytes()).decode_utf8() {
                    Ok(param_weather_station_id) => match param_weather_station_id.parse::<i32>() {
                        Ok(param_weather_station_id) => param_weather_station_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter WeatherStationId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["WeatherStationId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_measurement_date = match percent_encoding::percent_decode(path_params["MeasurementDate"].as_bytes()).decode_utf8() {
                    Ok(param_measurement_date) => match param_measurement_date.parse::<String>() {
                        Ok(param_measurement_date) => param_measurement_date,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter MeasurementDate: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["MeasurementDate"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_one_weather_data(
                                            param_weather_station_id,
                                            param_measurement_date,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOneWeatherDataResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_WEATHER_DATA_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneWeatherDataResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_WEATHER_DATA_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneWeatherDataResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetOneWeatherDataResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetOneWeatherDataResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetOneWeatherDataResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_WEATHER_DATA_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PatchOneWeatherData - PATCH /weather/data/{WeatherStationId}/{MeasurementDate}
            &hyper::Method::PATCH if path.matched(paths::ID_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE in set but failed match against \"{}\"", path, paths::REGEX_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE.as_str())
                    );

                let param_weather_station_id = match percent_encoding::percent_decode(path_params["WeatherStationId"].as_bytes()).decode_utf8() {
                    Ok(param_weather_station_id) => match param_weather_station_id.parse::<i32>() {
                        Ok(param_weather_station_id) => param_weather_station_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter WeatherStationId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["WeatherStationId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_measurement_date = match percent_encoding::percent_decode(path_params["MeasurementDate"].as_bytes()).decode_utf8() {
                    Ok(param_measurement_date) => match param_measurement_date.parse::<String>() {
                        Ok(param_measurement_date) => param_measurement_date,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter MeasurementDate: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["MeasurementDate"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_weather_data_patch: Option<models::WeatherDataPatch> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_weather_data_patch) => param_weather_data_patch,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter WeatherDataPatch - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter WeatherDataPatch due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_weather_data_patch = match param_weather_data_patch {
                                    Some(param_weather_data_patch) => param_weather_data_patch,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter WeatherDataPatch"))
                                                        .expect("Unable to create Bad Request response for missing body parameter WeatherDataPatch")),
                                };

                                let result = api_impl.patch_one_weather_data(
                                            param_weather_station_id,
                                            param_measurement_date,
                                            param_weather_data_patch,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PatchOneWeatherDataResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_WEATHER_DATA_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PatchOneWeatherDataResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_WEATHER_DATA_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PatchOneWeatherDataResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PatchOneWeatherDataResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PatchOneWeatherDataResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                PatchOneWeatherDataResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_WEATHER_DATA_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter WeatherDataPatch: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter WeatherDataPatch")),
                        }
            },

            // PostWeatherData - POST /weather/data
            &hyper::Method::POST if path.matched(paths::ID_WEATHER_DATA) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_weather_data: Option<models::WeatherData> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_weather_data) => param_weather_data,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter WeatherData - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter WeatherData due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_weather_data = match param_weather_data {
                                    Some(param_weather_data) => param_weather_data,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter WeatherData"))
                                                        .expect("Unable to create Bad Request response for missing body parameter WeatherData")),
                                };

                                let result = api_impl.post_weather_data(
                                            param_weather_data,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostWeatherDataResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_WEATHER_DATA_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostWeatherDataResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_WEATHER_DATA_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostWeatherDataResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostWeatherDataResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostWeatherDataResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_WEATHER_DATA_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter WeatherData: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter WeatherData")),
                        }
            },

            // DeleteOneWeatherStation - DELETE /weather/station/{WeatherStationId}
            &hyper::Method::DELETE if path.matched(paths::ID_WEATHER_STATION_WEATHERSTATIONID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_WEATHER_STATION_WEATHERSTATIONID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE WEATHER_STATION_WEATHERSTATIONID in set but failed match against \"{}\"", path, paths::REGEX_WEATHER_STATION_WEATHERSTATIONID.as_str())
                    );

                let param_weather_station_id = match percent_encoding::percent_decode(path_params["WeatherStationId"].as_bytes()).decode_utf8() {
                    Ok(param_weather_station_id) => match param_weather_station_id.parse::<i32>() {
                        Ok(param_weather_station_id) => param_weather_station_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter WeatherStationId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["WeatherStationId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_one_weather_station(
                                            param_weather_station_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOneWeatherStationResponse::SuccessfullyDeletedWeatherStation
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                DeleteOneWeatherStationResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_WEATHER_STATION_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteOneWeatherStationResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                DeleteOneWeatherStationResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                DeleteOneWeatherStationResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                DeleteOneWeatherStationResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_ONE_WEATHER_STATION_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAllWeatherStations - GET /weather/station
            &hyper::Method::GET if path.matched(paths::ID_WEATHER_STATION) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_bundesland = query_params.iter().filter(|e| e.0 == "bundesland").map(|e| e.1.to_owned())
                    .filter_map(|param_bundesland| param_bundesland.parse().ok())
                    .collect::<Vec<_>>();
                let param_bundesland = if !param_bundesland.is_empty() {
                    Some(param_bundesland)
                } else {
                    None
                };
                let param_name = query_params.iter().filter(|e| e.0 == "name").map(|e| e.1.to_owned())
                    .filter_map(|param_name| param_name.parse().ok())
                    .collect::<Vec<_>>();
                let param_name = if !param_name.is_empty() {
                    Some(param_name)
                } else {
                    None
                };
                let param_box2d = query_params.iter().filter(|e| e.0 == "box2d").map(|e| e.1.to_owned())
                    .filter_map(|param_box2d| param_box2d.parse().ok())
                    .collect::<Vec<_>>();
                let param_box2d = if !param_box2d.is_empty() {
                    Some(param_box2d)
                } else {
                    None
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_all_weather_stations(
                                            param_bundesland.as_ref(),
                                            param_name.as_ref(),
                                            param_box2d.as_ref(),
                                            param_offset,
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAllWeatherStationsResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_WEATHER_STATIONS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllWeatherStationsResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_WEATHER_STATIONS_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAllWeatherStationsResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetAllWeatherStationsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetAllWeatherStationsResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_WEATHER_STATIONS_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOneWeatherStation - GET /weather/station/{WeatherStationId}
            &hyper::Method::GET if path.matched(paths::ID_WEATHER_STATION_WEATHERSTATIONID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_WEATHER_STATION_WEATHERSTATIONID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE WEATHER_STATION_WEATHERSTATIONID in set but failed match against \"{}\"", path, paths::REGEX_WEATHER_STATION_WEATHERSTATIONID.as_str())
                    );

                let param_weather_station_id = match percent_encoding::percent_decode(path_params["WeatherStationId"].as_bytes()).decode_utf8() {
                    Ok(param_weather_station_id) => match param_weather_station_id.parse::<i32>() {
                        Ok(param_weather_station_id) => param_weather_station_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter WeatherStationId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["WeatherStationId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_one_weather_station(
                                            param_weather_station_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOneWeatherStationResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_WEATHER_STATION_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneWeatherStationResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_WEATHER_STATION_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetOneWeatherStationResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetOneWeatherStationResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                GetOneWeatherStationResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                GetOneWeatherStationResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ONE_WEATHER_STATION_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // PatchOneWeatherStation - PATCH /weather/station/{WeatherStationId}
            &hyper::Method::PATCH if path.matched(paths::ID_WEATHER_STATION_WEATHERSTATIONID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_WEATHER_STATION_WEATHERSTATIONID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE WEATHER_STATION_WEATHERSTATIONID in set but failed match against \"{}\"", path, paths::REGEX_WEATHER_STATION_WEATHERSTATIONID.as_str())
                    );

                let param_weather_station_id = match percent_encoding::percent_decode(path_params["WeatherStationId"].as_bytes()).decode_utf8() {
                    Ok(param_weather_station_id) => match param_weather_station_id.parse::<i32>() {
                        Ok(param_weather_station_id) => param_weather_station_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter WeatherStationId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["WeatherStationId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_weather_station_patch: Option<models::WeatherStationPatch> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_weather_station_patch) => param_weather_station_patch,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter WeatherStationPatch - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter WeatherStationPatch due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_weather_station_patch = match param_weather_station_patch {
                                    Some(param_weather_station_patch) => param_weather_station_patch,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter WeatherStationPatch"))
                                                        .expect("Unable to create Bad Request response for missing body parameter WeatherStationPatch")),
                                };

                                let result = api_impl.patch_one_weather_station(
                                            param_weather_station_id,
                                            param_weather_station_patch,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PatchOneWeatherStationResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_WEATHER_STATION_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PatchOneWeatherStationResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_WEATHER_STATION_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PatchOneWeatherStationResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PatchOneWeatherStationResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PatchOneWeatherStationResponse::TheSpecifiedResourceWasNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                PatchOneWeatherStationResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for PATCH_ONE_WEATHER_STATION_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter WeatherStationPatch: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter WeatherStationPatch")),
                        }
            },

            // PostWeatherStation - POST /weather/station
            &hyper::Method::POST if path.matched(paths::ID_WEATHER_STATION) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_weather_station: Option<models::WeatherStation> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_weather_station) => param_weather_station,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter WeatherStation - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter WeatherStation due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_weather_station = match param_weather_station {
                                    Some(param_weather_station) => param_weather_station,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter WeatherStation"))
                                                        .expect("Unable to create Bad Request response for missing body parameter WeatherStation")),
                                };

                                let result = api_impl.post_weather_station(
                                            param_weather_station,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostWeatherStationResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_WEATHER_STATION_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostWeatherStationResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_WEATHER_STATION_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                PostWeatherStationResponse::MissingAPIKey
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                PostWeatherStationResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                },
                                                PostWeatherStationResponse::BackendError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for POST_WEATHER_STATION_BACKEND_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter WeatherStation: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter WeatherStation")),
                        }
            },

            _ if path.matched(paths::ID_FIELD) => method_not_allowed(),
            _ if path.matched(paths::ID_FIELD_FIELDID) => method_not_allowed(),
            _ if path.matched(paths::ID_FIELD_FIELDID_PLANTER) => method_not_allowed(),
            _ if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID) => method_not_allowed(),
            _ if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT) => method_not_allowed(),
            _ if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID) => method_not_allowed(),
            _ if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT) => method_not_allowed(),
            _ if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT_QUALITYMEASUREMENTID) => method_not_allowed(),
            _ if path.matched(paths::ID_FORECAST) => method_not_allowed(),
            _ if path.matched(paths::ID_FORECAST_FORECASTID) => method_not_allowed(),
            _ if path.matched(paths::ID_HARVEST_COMPLETE) => method_not_allowed(),
            _ if path.matched(paths::ID_HARVEST_COMPLETE_HARVESTTASKID) => method_not_allowed(),
            _ if path.matched(paths::ID_HARVEST_TASK) => method_not_allowed(),
            _ if path.matched(paths::ID_HARVEST_TASK_HARVESTTASKID) => method_not_allowed(),
            _ if path.matched(paths::ID_SPRAY_COMPLETE) => method_not_allowed(),
            _ if path.matched(paths::ID_SPRAY_COMPLETE_SPRAYTASKID) => method_not_allowed(),
            _ if path.matched(paths::ID_SPRAY_TASK) => method_not_allowed(),
            _ if path.matched(paths::ID_SPRAY_TASK_SPRAYTASKID) => method_not_allowed(),
            _ if path.matched(paths::ID_TERRAIN) => method_not_allowed(),
            _ if path.matched(paths::ID_TERRAIN_TERRAINID) => method_not_allowed(),
            _ if path.matched(paths::ID_TOKEN) => method_not_allowed(),
            _ if path.matched(paths::ID_TOKEN_USERID) => method_not_allowed(),
            _ if path.matched(paths::ID_TOKEN_USERID_TOKENSTR) => method_not_allowed(),
            _ if path.matched(paths::ID_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT) => method_not_allowed(),
            _ if path.matched(paths::ID_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT_ACCESSRIGHTPATH) => method_not_allowed(),
            _ if path.matched(paths::ID_VIVC) => method_not_allowed(),
            _ if path.matched(paths::ID_VIVC_CULTIVARID) => method_not_allowed(),
            _ if path.matched(paths::ID_WEATHER_DATA) => method_not_allowed(),
            _ if path.matched(paths::ID_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE) => method_not_allowed(),
            _ if path.matched(paths::ID_WEATHER_STATION) => method_not_allowed(),
            _ if path.matched(paths::ID_WEATHER_STATION_WEATHERSTATIONID) => method_not_allowed(),
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response"))
        }
        }
        Box::pin(run(self.api_impl.clone(), req))
    }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Result<&'static str, ()> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match request.method() {
            // DeleteOneCultivar - DELETE /vivc/{CultivarId}
            &hyper::Method::DELETE if path.matched(paths::ID_VIVC_CULTIVARID) => Ok("DeleteOneCultivar"),
            // GetAllCultivars - GET /vivc
            &hyper::Method::GET if path.matched(paths::ID_VIVC) => Ok("GetAllCultivars"),
            // GetOneCultivar - GET /vivc/{CultivarId}
            &hyper::Method::GET if path.matched(paths::ID_VIVC_CULTIVARID) => Ok("GetOneCultivar"),
            // PatchOneCultivar - PATCH /vivc/{CultivarId}
            &hyper::Method::PATCH if path.matched(paths::ID_VIVC_CULTIVARID) => Ok("PatchOneCultivar"),
            // PostCultivar - POST /vivc
            &hyper::Method::POST if path.matched(paths::ID_VIVC) => Ok("PostCultivar"),
            // DeleteField - DELETE /field
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD) => Ok("DeleteField"),
            // DeleteOneField - DELETE /field/{FieldId}
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD_FIELDID) => Ok("DeleteOneField"),
            // GetAllFields - GET /field
            &hyper::Method::GET if path.matched(paths::ID_FIELD) => Ok("GetAllFields"),
            // GetOneField - GET /field/{FieldId}
            &hyper::Method::GET if path.matched(paths::ID_FIELD_FIELDID) => Ok("GetOneField"),
            // PatchOneField - PATCH /field/{FieldId}
            &hyper::Method::PATCH if path.matched(paths::ID_FIELD_FIELDID) => Ok("PatchOneField"),
            // PostField - POST /field
            &hyper::Method::POST if path.matched(paths::ID_FIELD) => Ok("PostField"),
            // DeleteForecast - DELETE /forecast
            &hyper::Method::DELETE if path.matched(paths::ID_FORECAST) => Ok("DeleteForecast"),
            // DeleteOneForecast - DELETE /forecast/{ForecastId}
            &hyper::Method::DELETE if path.matched(paths::ID_FORECAST_FORECASTID) => Ok("DeleteOneForecast"),
            // GetAllForecasts - GET /forecast
            &hyper::Method::GET if path.matched(paths::ID_FORECAST) => Ok("GetAllForecasts"),
            // GetOneForecast - GET /forecast/{ForecastId}
            &hyper::Method::GET if path.matched(paths::ID_FORECAST_FORECASTID) => Ok("GetOneForecast"),
            // PostForecast - POST /forecast
            &hyper::Method::POST if path.matched(paths::ID_FORECAST) => Ok("PostForecast"),
            // DeleteCompletedHarvestTask - DELETE /harvest/complete
            &hyper::Method::DELETE if path.matched(paths::ID_HARVEST_COMPLETE) => Ok("DeleteCompletedHarvestTask"),
            // DeleteOneCompletedHarvestTask - DELETE /harvest/complete/{HarvestTaskId}
            &hyper::Method::DELETE if path.matched(paths::ID_HARVEST_COMPLETE_HARVESTTASKID) => Ok("DeleteOneCompletedHarvestTask"),
            // DeleteOneScheduledHarvestTask - DELETE /harvest/task/{HarvestTaskId}
            &hyper::Method::DELETE if path.matched(paths::ID_HARVEST_TASK_HARVESTTASKID) => Ok("DeleteOneScheduledHarvestTask"),
            // DeleteScheduledHarvestTask - DELETE /harvest/task
            &hyper::Method::DELETE if path.matched(paths::ID_HARVEST_TASK) => Ok("DeleteScheduledHarvestTask"),
            // GetAllCompletedHarvestTasks - GET /harvest/complete
            &hyper::Method::GET if path.matched(paths::ID_HARVEST_COMPLETE) => Ok("GetAllCompletedHarvestTasks"),
            // GetAllScheduledHarvestTasks - GET /harvest/task
            &hyper::Method::GET if path.matched(paths::ID_HARVEST_TASK) => Ok("GetAllScheduledHarvestTasks"),
            // GetOneCompletedHarvestTask - GET /harvest/complete/{HarvestTaskId}
            &hyper::Method::GET if path.matched(paths::ID_HARVEST_COMPLETE_HARVESTTASKID) => Ok("GetOneCompletedHarvestTask"),
            // GetOneScheduledHarvestTask - GET /harvest/task/{HarvestTaskId}
            &hyper::Method::GET if path.matched(paths::ID_HARVEST_TASK_HARVESTTASKID) => Ok("GetOneScheduledHarvestTask"),
            // PostCompletedHarvestTask - POST /harvest/complete
            &hyper::Method::POST if path.matched(paths::ID_HARVEST_COMPLETE) => Ok("PostCompletedHarvestTask"),
            // PostScheduledHarvestTask - POST /harvest/task
            &hyper::Method::POST if path.matched(paths::ID_HARVEST_TASK) => Ok("PostScheduledHarvestTask"),
            // DeleteOnePlant - DELETE /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID) => Ok("DeleteOnePlant"),
            // DeletePlant - DELETE /field/{FieldId}/planter/{PlanterRunId}/plant
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT) => Ok("DeletePlant"),
            // GetAllPlants - GET /field/{FieldId}/planter/{PlanterRunId}/plant
            &hyper::Method::GET if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT) => Ok("GetAllPlants"),
            // GetOnePlant - GET /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}
            &hyper::Method::GET if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID) => Ok("GetOnePlant"),
            // PatchOnePlant - PATCH /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}
            &hyper::Method::PATCH if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID) => Ok("PatchOnePlant"),
            // PostPlant - POST /field/{FieldId}/planter/{PlanterRunId}/plant
            &hyper::Method::POST if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT) => Ok("PostPlant"),
            // DeleteOnePlanterRun - DELETE /field/{FieldId}/planter/{PlanterRunId}
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID) => Ok("DeleteOnePlanterRun"),
            // DeletePlanterRun - DELETE /field/{FieldId}/planter
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD_FIELDID_PLANTER) => Ok("DeletePlanterRun"),
            // GetAllPlanterRuns - GET /field/{FieldId}/planter
            &hyper::Method::GET if path.matched(paths::ID_FIELD_FIELDID_PLANTER) => Ok("GetAllPlanterRuns"),
            // GetOnePlanterRun - GET /field/{FieldId}/planter/{PlanterRunId}
            &hyper::Method::GET if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID) => Ok("GetOnePlanterRun"),
            // PostPlanterRun - POST /field/{FieldId}/planter
            &hyper::Method::POST if path.matched(paths::ID_FIELD_FIELDID_PLANTER) => Ok("PostPlanterRun"),
            // DeleteOneQualityMeasurement - DELETE /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement/{QualityMeasurementId}
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT_QUALITYMEASUREMENTID) => Ok("DeleteOneQualityMeasurement"),
            // DeleteQualityMeasurement - DELETE /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement
            &hyper::Method::DELETE if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT) => Ok("DeleteQualityMeasurement"),
            // GetAllQualityMeasurements - GET /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement
            &hyper::Method::GET if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT) => Ok("GetAllQualityMeasurements"),
            // GetOneQualityMeasurement - GET /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement/{QualityMeasurementId}
            &hyper::Method::GET if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT_QUALITYMEASUREMENTID) => Ok("GetOneQualityMeasurement"),
            // PostQualityMeasurement - POST /field/{FieldId}/planter/{PlanterRunId}/plant/{PlantId}/quality_measurement
            &hyper::Method::POST if path.matched(paths::ID_FIELD_FIELDID_PLANTER_PLANTERRUNID_PLANT_PLANTID_QUALITY_MEASUREMENT) => Ok("PostQualityMeasurement"),
            // DeleteCompletedSprayTask - DELETE /spray/complete
            &hyper::Method::DELETE if path.matched(paths::ID_SPRAY_COMPLETE) => Ok("DeleteCompletedSprayTask"),
            // DeleteOneCompletedSprayTask - DELETE /spray/complete/{SprayTaskId}
            &hyper::Method::DELETE if path.matched(paths::ID_SPRAY_COMPLETE_SPRAYTASKID) => Ok("DeleteOneCompletedSprayTask"),
            // DeleteOneScheduledSprayTask - DELETE /spray/task/{SprayTaskId}
            &hyper::Method::DELETE if path.matched(paths::ID_SPRAY_TASK_SPRAYTASKID) => Ok("DeleteOneScheduledSprayTask"),
            // DeleteScheduledSprayTask - DELETE /spray/task
            &hyper::Method::DELETE if path.matched(paths::ID_SPRAY_TASK) => Ok("DeleteScheduledSprayTask"),
            // GetAllCompletedSprayTasks - GET /spray/complete
            &hyper::Method::GET if path.matched(paths::ID_SPRAY_COMPLETE) => Ok("GetAllCompletedSprayTasks"),
            // GetAllScheduledSprayTasks - GET /spray/task
            &hyper::Method::GET if path.matched(paths::ID_SPRAY_TASK) => Ok("GetAllScheduledSprayTasks"),
            // GetOneCompletedSprayTask - GET /spray/complete/{SprayTaskId}
            &hyper::Method::GET if path.matched(paths::ID_SPRAY_COMPLETE_SPRAYTASKID) => Ok("GetOneCompletedSprayTask"),
            // GetOneScheduledSprayTask - GET /spray/task/{SprayTaskId}
            &hyper::Method::GET if path.matched(paths::ID_SPRAY_TASK_SPRAYTASKID) => Ok("GetOneScheduledSprayTask"),
            // PostCompletedSprayTask - POST /spray/complete
            &hyper::Method::POST if path.matched(paths::ID_SPRAY_COMPLETE) => Ok("PostCompletedSprayTask"),
            // PostScheduledSprayTask - POST /spray/task
            &hyper::Method::POST if path.matched(paths::ID_SPRAY_TASK) => Ok("PostScheduledSprayTask"),
            // DeleteOneTerrain - DELETE /terrain/{TerrainId}
            &hyper::Method::DELETE if path.matched(paths::ID_TERRAIN_TERRAINID) => Ok("DeleteOneTerrain"),
            // DeleteTerrain - DELETE /terrain
            &hyper::Method::DELETE if path.matched(paths::ID_TERRAIN) => Ok("DeleteTerrain"),
            // GetAllTerrains - GET /terrain
            &hyper::Method::GET if path.matched(paths::ID_TERRAIN) => Ok("GetAllTerrains"),
            // GetOneTerrain - GET /terrain/{TerrainId}
            &hyper::Method::GET if path.matched(paths::ID_TERRAIN_TERRAINID) => Ok("GetOneTerrain"),
            // DeleteOneAccessRight - DELETE /token/{UserId}/{Tokenstr}/access_right/{AccessRightPath}
            &hyper::Method::DELETE if path.matched(paths::ID_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT_ACCESSRIGHTPATH) => Ok("DeleteOneAccessRight"),
            // DeleteOneToken - DELETE /token/{UserId}/{Tokenstr}
            &hyper::Method::DELETE if path.matched(paths::ID_TOKEN_USERID_TOKENSTR) => Ok("DeleteOneToken"),
            // DeleteOneUser - DELETE /token/{UserId}
            &hyper::Method::DELETE if path.matched(paths::ID_TOKEN_USERID) => Ok("DeleteOneUser"),
            // GetAccessRights - GET /token/{UserId}/{Tokenstr}/access_right
            &hyper::Method::GET if path.matched(paths::ID_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT) => Ok("GetAccessRights"),
            // GetAllTokens - GET /token
            &hyper::Method::GET if path.matched(paths::ID_TOKEN) => Ok("GetAllTokens"),
            // GetOneAccessRight - GET /token/{UserId}/{Tokenstr}/access_right/{AccessRightPath}
            &hyper::Method::GET if path.matched(paths::ID_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT_ACCESSRIGHTPATH) => Ok("GetOneAccessRight"),
            // GetUserTokens - GET /token/{UserId}
            &hyper::Method::GET if path.matched(paths::ID_TOKEN_USERID) => Ok("GetUserTokens"),
            // PostAccessRight - POST /token/{UserId}/{Tokenstr}/access_right
            &hyper::Method::POST if path.matched(paths::ID_TOKEN_USERID_TOKENSTR_ACCESS_RIGHT) => Ok("PostAccessRight"),
            // PostToken - POST /token
            &hyper::Method::POST if path.matched(paths::ID_TOKEN) => Ok("PostToken"),
            // PostUserToken - POST /token/{UserId}
            &hyper::Method::POST if path.matched(paths::ID_TOKEN_USERID) => Ok("PostUserToken"),
            // DeleteOneWeatherData - DELETE /weather/data/{WeatherStationId}/{MeasurementDate}
            &hyper::Method::DELETE if path.matched(paths::ID_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE) => Ok("DeleteOneWeatherData"),
            // GetAllWeatherData - GET /weather/data
            &hyper::Method::GET if path.matched(paths::ID_WEATHER_DATA) => Ok("GetAllWeatherData"),
            // GetOneWeatherData - GET /weather/data/{WeatherStationId}/{MeasurementDate}
            &hyper::Method::GET if path.matched(paths::ID_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE) => Ok("GetOneWeatherData"),
            // PatchOneWeatherData - PATCH /weather/data/{WeatherStationId}/{MeasurementDate}
            &hyper::Method::PATCH if path.matched(paths::ID_WEATHER_DATA_WEATHERSTATIONID_MEASUREMENTDATE) => Ok("PatchOneWeatherData"),
            // PostWeatherData - POST /weather/data
            &hyper::Method::POST if path.matched(paths::ID_WEATHER_DATA) => Ok("PostWeatherData"),
            // DeleteOneWeatherStation - DELETE /weather/station/{WeatherStationId}
            &hyper::Method::DELETE if path.matched(paths::ID_WEATHER_STATION_WEATHERSTATIONID) => Ok("DeleteOneWeatherStation"),
            // GetAllWeatherStations - GET /weather/station
            &hyper::Method::GET if path.matched(paths::ID_WEATHER_STATION) => Ok("GetAllWeatherStations"),
            // GetOneWeatherStation - GET /weather/station/{WeatherStationId}
            &hyper::Method::GET if path.matched(paths::ID_WEATHER_STATION_WEATHERSTATIONID) => Ok("GetOneWeatherStation"),
            // PatchOneWeatherStation - PATCH /weather/station/{WeatherStationId}
            &hyper::Method::PATCH if path.matched(paths::ID_WEATHER_STATION_WEATHERSTATIONID) => Ok("PatchOneWeatherStation"),
            // PostWeatherStation - POST /weather/station
            &hyper::Method::POST if path.matched(paths::ID_WEATHER_STATION) => Ok("PostWeatherStation"),
            _ => Err(()),
        }
    }
}
