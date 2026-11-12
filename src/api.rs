use super::server::*;
use crate::response::{date_to_str, Delete, FromJson, Get, Search, Upsert};
use async_trait::async_trait;
use auth_backend::db::find_token_with_user;
use chrono::NaiveDate;
use diesel_ext::db_backend::{Creatable, DbConnector};
use digivine_api_postgres::{
    field::FieldFilter,
    weather::{WeatherData, WeatherDataFilter, WeatherStationFilter},
    RelationalBackendLibError,
};
use digivine_api_vivc::db::CultivarFilter;
use digivine_server::{models::Forecast, *};
use macros_lib::{delete, delete_all, get, get_all, init, patch, post, search};
use swagger::{ApiError, Has, XSpanIdString};

const DEFAULT_OFFSET: i32 = 0;
const DEFAULT_LIMIT: i32 = 100;

macro_rules! parse {
    ($ty:path, $e:expr) => {
        match <$ty>::from_json($e) {
            Ok(v) => v,
            Err(msg) => return Ok(msg.as_bad_request()),
        }
    };
}

#[async_trait]
impl<C, Conn: DbConnector> Api<C> for Server<C, Conn>
where
    C: Has<XSpanIdString> + Send + Sync,
{
    /*
       ______                           _
      |  ____|                         | |
      | |__ ___  _ __ ___  ___ __ _ ___| |_
      |  __/ _ \| '__/ _ \/ __/ _` / __| __|
      | | | (_) | | |  __/ (_| (_| \__ \ |_
      |_|  \___/|_|  \___|\___\__,_|___/\__|

    */
    async fn get_all_forecasts(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetAllForecastsResponse, ApiError> {
        get_all!(relational_backend)
    }

    async fn get_one_forecast(
        &self,
        id: i64,
        _context: &C,
    ) -> Result<GetOneForecastResponse, ApiError> {
        get!(relational_backend)
    }

    async fn post_forecast(
        &self,
        insert: Forecast,
        _context: &C,
    ) -> Result<PostForecastResponse, ApiError> {
        post!(relational_backend)
    }

    async fn delete_forecast(&self, _context: &C) -> Result<DeleteForecastResponse, ApiError> {
        Ok(DeleteForecastResponse::MethodNotAllowed)
    }

    async fn delete_one_forecast(
        &self,
        id: i64,
        _context: &C,
    ) -> Result<DeleteOneForecastResponse, ApiError> {
        delete!(relational_backend)
    }

    /*
        _____ _      _     _
        |  ___(_) ___| | __| |
        | |_  | |/ _ \ |/ _` |
        |  _| | |  __/ | (_| |
        |_|   |_|\___|_|\__,_|

    */
    async fn get_all_fields(
        &self,
        box2d: Option<&Vec<f64>>,
        fuzzy_field_keywords: Option<&Vec<String>>,
        field_name_contains: Option<&Vec<String>>,
        fruit_name_contains: Option<&Vec<String>>,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetAllFieldsResponse, ApiError> {
        debug!(
        "Receiving get_all_fields request with parameters: box2d: {:?}, fuzzy_field_keywords: {:?}, field_name_contains: {:?}, fruit_name_contains: {:?}, offset: {:?}, limit: {:?}",
        box2d, fuzzy_field_keywords, field_name_contains, fruit_name_contains, offset, limit
        );
        let field_filter = FieldFilter::new()
            .with_field_names(field_name_contains)
            .with_fruit_names(fruit_name_contains)
            .with_fuzzy(fuzzy_field_keywords);
        search!(relational_backend, filter = (field_filter, box2d))
    }

    async fn get_one_field(&self, id: i64, _context: &C) -> Result<GetOneFieldResponse, ApiError> {
        get!(relational_backend)
    }

    async fn post_field(
        &self,
        insert: models::Field,
        _context: &C,
    ) -> Result<PostFieldResponse, ApiError> {
        post!(relational_backend)
    }

    async fn delete_field(&self, _context: &C) -> Result<DeleteFieldResponse, ApiError> {
        Ok(DeleteFieldResponse::MethodNotAllowed)
    }

    async fn delete_one_field(
        &self,
        id: i64,
        _context: &C,
    ) -> Result<DeleteOneFieldResponse, ApiError> {
        delete!(relational_backend)
    }

    async fn patch_one_field(
        &self,
        id: i64,
        patch: models::FieldPatch,
        _context: &C,
    ) -> Result<PatchOneFieldResponse, ApiError> {
        patch!(relational_backend)
    }

    /*
         ____  _             _
        |  _ \| | __ _ _ __ | |_ ___ _ __
        | |_) | |/ _` | '_ \| __/ _ \ '__|
        |  __/| | (_| | | | | ||  __/ |
        |_|   |_|\__,_|_| |_|\__\___|_|

    */
    async fn get_all_planter_runs(
        &self,
        field_id: i64,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetAllPlanterRunsResponse, ApiError> {
        search!(relational_backend, filter = (Some(field_id), box2d))
    }

    async fn post_planter_run(
        &self,
        field_id: i64,
        mut insert: models::PlanterRun,
        _context: &C,
    ) -> Result<PostPlanterRunResponse, ApiError> {
        insert.properties.field_id = field_id;
        post!(relational_backend)
    }

    async fn delete_one_planter_run(
        &self,
        _field_id: i64,
        planter_run_id: i64,
        _context: &C,
    ) -> Result<DeleteOnePlanterRunResponse, ApiError> {
        delete!(relational_backend, id = planter_run_id)
    }

    async fn get_one_planter_run(
        &self,
        _field_id: i64,
        planter_run_id: i64,
        _context: &C,
    ) -> Result<GetOnePlanterRunResponse, ApiError> {
        get!(relational_backend, id = planter_run_id)
    }

    async fn delete_planter_run(
        &self,
        _field_id: i64,
        _context: &C,
    ) -> Result<DeletePlanterRunResponse, ApiError> {
        Ok(DeletePlanterRunResponse::MethodNotAllowed)
    }

    /*
         ____  _             _
        |  _ \| | __ _ _ __ | |_
        | |_) | |/ _` | '_ \| __|
        |  __/| | (_| | | | | |_
        |_|   |_|\__,_|_| |_|\__|
    */
    async fn get_all_plants(
        &self,
        _field_id: i64,
        plantrun_id: i64,
        offset: Option<i32>,
        limit: Option<i32>,
        box2d: Option<&Vec<f64>>,
        _context: &C,
    ) -> Result<GetAllPlantsResponse, ApiError> {
        search!(relational_backend, filter = (Some(plantrun_id), box2d))
    }

    async fn get_one_plant(
        &self,
        _field_id: i64,
        _plantrun_id: i64,
        plant_id: i64,
        _context: &C,
    ) -> Result<GetOnePlantResponse, ApiError> {
        get!(relational_backend, id = plant_id)
    }

    async fn post_plant(
        &self,
        _field_id: i64,
        plantrun_id: i64,
        plant: models::Plant,
        _context: &C,
    ) -> Result<PostPlantResponse, ApiError> {
        post!(relational_backend, insert = (plantrun_id, plant))
    }

    async fn delete_one_plant(
        &self,
        _field_id: i64,
        _plantrun_id: i64,
        plant_id: i64,
        _context: &C,
    ) -> Result<DeleteOnePlantResponse, ApiError> {
        delete!(relational_backend, id = plant_id)
    }

    async fn delete_plant(
        &self,
        _field_id: i64,
        _plantrun_id: i64,
        _context: &C,
    ) -> Result<DeletePlantResponse, ApiError> {
        Ok(DeletePlantResponse::MethodNotAllowed)
    }

    async fn patch_one_plant(
        &self,
        _: i64,
        _: i64,
        id: i64,
        patch: models::PlantPatch,
        _context: &C,
    ) -> Result<PatchOnePlantResponse, ApiError> {
        patch!(relational_backend)
    }

    /*

       _    _                           _
      | |  | |                         | |
      | |__| | __ _ _ ____   _____  ___| |_
      |  __  |/ _` | '__\ \ / / _ \/ __| __|
      | |  | | (_| | |   \ V /  __/\__ \ |_
      |_|  |_|\__,_|_|    \_/ \___||___/\__|


    */
    /// Can't delete complete collection
    async fn delete_completed_harvest_task(
        &self,
        _context: &C,
    ) -> Result<DeleteCompletedHarvestTaskResponse, ApiError> {
        Ok(DeleteCompletedHarvestTaskResponse::MethodNotAllowed)
    }

    /// Removes either a specific stored Harvest Task or a specific stored Harvest with Harvest Measurements from the completed collection
    async fn delete_one_completed_harvest_task(
        &self,
        id: i64,
        _context: &C,
    ) -> Result<DeleteOneCompletedHarvestTaskResponse, ApiError> {
        delete!(relational_backend)
    }

    /// Removes a specific stored Harvest Task from the scheduled collection
    async fn delete_one_scheduled_harvest_task(
        &self,
        id: i64,
        _context: &C,
    ) -> Result<DeleteOneScheduledHarvestTaskResponse, ApiError> {
        delete!(relational_backend)
    }

    /// Can't delete complete collection
    async fn delete_scheduled_harvest_task(
        &self,
        _context: &C,
    ) -> Result<DeleteScheduledHarvestTaskResponse, ApiError> {
        Ok(DeleteScheduledHarvestTaskResponse::MethodNotAllowed)
    }

    /// Returns a list of (all) stored completed Harvest Tasks and Harvests with Harvest Measurements
    async fn get_all_completed_harvest_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetAllCompletedHarvestTasksResponse, ApiError> {
        search!(relational_backend, filter = box2d)
    }

    /// Returns a list of (all) stored scheduled Harvest Tasks
    async fn get_all_scheduled_harvest_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetAllScheduledHarvestTasksResponse, ApiError> {
        search!(relational_backend, filter = box2d)
    }

    /// Returns either a specific stored Harvest Task or a specific stored Harvest with Harvest Measurements from the completed collection
    async fn get_one_completed_harvest_task(
        &self,
        id: i64,
        _context: &C,
    ) -> Result<GetOneCompletedHarvestTaskResponse, ApiError> {
        get!(relational_backend)
    }

    /// Returns a specific stored Harvest Task from the scheduled collection
    async fn get_one_scheduled_harvest_task(
        &self,
        id: i64,
        _context: &C,
    ) -> Result<GetOneScheduledHarvestTaskResponse, ApiError> {
        get!(relational_backend)
    }

    /// Adds either a new Harvest Task or a new Harvest with Harvest Measurements to the completed collection
    async fn post_completed_harvest_task(
        &self,
        insert: models::HarvestOrTask,
        _context: &C,
    ) -> Result<PostCompletedHarvestTaskResponse, ApiError> {
        post!(relational_backend)
    }

    /// Adds a new Harvest Task to the scheduled collection
    async fn post_scheduled_harvest_task(
        &self,
        insert: models::HarvestTask,
        _context: &C,
    ) -> Result<PostScheduledHarvestTaskResponse, ApiError> {
        post!(relational_backend)
    }

    /*

      _____
     / ____|
    | (___  _ __  _ __ __ _ _   _
     \___ \| '_ \| '__/ _` | | | |
     ____) | |_) | | | (_| | |_| |
    |_____/| .__/|_|  \__,_|\__, |
           | |               __/ |
           |_|              |___/

      */
    /// Can't delete complete collection
    async fn delete_completed_spray_task(
        &self,
        _context: &C,
    ) -> Result<DeleteCompletedSprayTaskResponse, ApiError> {
        Ok(DeleteCompletedSprayTaskResponse::MethodNotAllowed)
    }

    /// Removes either a specific stored Spray Task or a specific stored Spray with Spray Measurements from the completed collection
    async fn delete_one_completed_spray_task(
        &self,
        id: i64,
        _context: &C,
    ) -> Result<DeleteOneCompletedSprayTaskResponse, ApiError> {
        delete!(relational_backend)
    }

    /// Removes a specific stored Spray Task from the scheduled collection
    async fn delete_one_scheduled_spray_task(
        &self,
        id: i64,
        _context: &C,
    ) -> Result<DeleteOneScheduledSprayTaskResponse, ApiError> {
        delete!(relational_backend)
    }

    /// Can't delete complete collection
    async fn delete_scheduled_spray_task(
        &self,
        _context: &C,
    ) -> Result<DeleteScheduledSprayTaskResponse, ApiError> {
        Ok(DeleteScheduledSprayTaskResponse::MethodNotAllowed)
    }

    /// Returns a list of (all) stored completed Spray Tasks and Sprays with Spray Measurements
    async fn get_all_completed_spray_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetAllCompletedSprayTasksResponse, ApiError> {
        search!(relational_backend, filter = box2d)
    }

    /// Returns a list of (all) stored scheduled Spray Tasks
    async fn get_all_scheduled_spray_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetAllScheduledSprayTasksResponse, ApiError> {
        search!(relational_backend, filter = box2d)
    }

    /// Returns either a specific stored Spray Task or a specific stored Spray with Spray Measurements from the completed collection
    async fn get_one_completed_spray_task(
        &self,
        id: i64,
        _context: &C,
    ) -> Result<GetOneCompletedSprayTaskResponse, ApiError> {
        get!(relational_backend)
    }

    /// Returns a specific stored Spray Task from the scheduled collection
    async fn get_one_scheduled_spray_task(
        &self,
        id: i64,
        _context: &C,
    ) -> Result<GetOneScheduledSprayTaskResponse, ApiError> {
        get!(relational_backend)
    }

    /// Adds either a new Spray Task or a new Spray with Spray Measurements to the completed collection
    async fn post_completed_spray_task(
        &self,
        insert: models::SprayTask,
        _context: &C,
    ) -> Result<PostCompletedSprayTaskResponse, ApiError> {
        post!(relational_backend)
    }

    /// Adds a new Spray Task to the scheduled collection
    async fn post_scheduled_spray_task(
        &self,
        insert: models::SprayTask,
        _context: &C,
    ) -> Result<PostScheduledSprayTaskResponse, ApiError> {
        post!(relational_backend)
    }

    /*
          ___              _ _ _         __  __                                                    _
         / _ \ _   _  __ _| (_) |_ _   _|  \/  | ___  __ _ ___ _   _ _ __ ___ _ __ ___   ___ _ __ | |_
        | | | | | | |/ _` | | | __| | | | |\/| |/ _ \/ _` / __| | | | '__/ _ \ '_ ` _ \ / _ \ '_ \| __|
        | |_| | |_| | (_| | | | |_| |_| | |  | |  __/ (_| \__ \ |_| | | |  __/ | | | | |  __/ | | | |_
         \__\_\\__,_|\__,_|_|_|\__|\__, |_|  |_|\___|\__,_|___/\__,_|_|  \___|_| |_| |_|\___|_| |_|\__|
                                |___/
    */
    async fn get_all_quality_measurements(
        &self,
        _field_id: i64,
        _plantrun_id: i64,
        plant_id: i64,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetAllQualityMeasurementsResponse, ApiError> {
        search!(relational_backend, filter = Some(plant_id))
    }

    async fn get_one_quality_measurement(
        &self,
        _field_id: i64,
        _plantrun_id: i64,
        _plant_id: i64,
        quality_measurement_id: i64,
        _context: &C,
    ) -> Result<GetOneQualityMeasurementResponse, ApiError> {
        get!(relational_backend, id = quality_measurement_id)
    }

    async fn post_quality_measurement(
        &self,
        _field_id: i64,
        _plantrun_id: i64,
        plant_id: i64,
        quality_measurement: models::QualityMeasurement,
        _context: &C,
    ) -> Result<PostQualityMeasurementResponse, ApiError> {
        post!(relational_backend, insert = (plant_id, quality_measurement))
    }

    async fn delete_quality_measurement(
        &self,
        _field_id: i64,
        _plantrun_id: i64,
        _plant_id: i64,
        _context: &C,
    ) -> Result<DeleteQualityMeasurementResponse, ApiError> {
        Ok(DeleteQualityMeasurementResponse::MethodNotAllowed)
    }

    async fn delete_one_quality_measurement(
        &self,
        _field_id: i64,
        _plantrun_id: i64,
        _plant_id: i64,
        quality_measurement_id: i64,
        _context: &C,
    ) -> Result<DeleteOneQualityMeasurementResponse, ApiError> {
        delete!(relational_backend, id = quality_measurement_id)
    }

    /*
         _____                   _
        |_   _|__ _ __ _ __ __ _(_)_ __
          | |/ _ \ '__| '__/ _` | | '_ \
          | |  __/ |  | | | (_| | | | | |
          |_|\___|_|  |_|  \__,_|_|_| |_|
    */
    async fn get_all_terrains(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetAllTerrainsResponse, ApiError> {
        get_all!(terrain_backend)
    }

    async fn get_one_terrain(
        &self,
        id: i64,
        _context: &C,
    ) -> Result<GetOneTerrainResponse, ApiError> {
        get!(terrain_backend)
    }

    async fn delete_terrain(&self, _context: &C) -> Result<DeleteTerrainResponse, ApiError> {
        Ok(DeleteTerrainResponse::MethodNotAllowed)
    }

    async fn delete_one_terrain(
        &self,
        id: i64,
        _context: &C,
    ) -> Result<DeleteOneTerrainResponse, ApiError> {
        delete!(terrain_backend)
    }

    /*
         _______    _
        |__   __|  | |
           | | ___ | | _____ _ __
           | |/ _ \| |/ / _ \ '_ \
           | | (_) |   <  __/ | | |
           |_|\___/|_|\_\___|_| |_|

    */
    async fn delete_one_access_right(
        &self,
        user_id: i32,
        tokenstr: String,
        access_right_path: String,
        _context: &C,
    ) -> Result<DeleteOneAccessRightResponse, ApiError> {
        init!(auth_backend);
        Ok(
            match find_token_with_user(&mut auth_backend, user_id, &tokenstr) {
                Ok(Some(tk)) => Delete::delete(&mut auth_backend, (tk.id, access_right_path)),
                Ok(None) => DeleteOneAccessRightResponse::TheSpecifiedResourceWasNotFound,
                Err(e) => e.as_backend_error_resp(),
            },
        )
    }

    async fn delete_one_token(
        &self,
        user_id: i32,
        tokenstr: String,
        _context: &C,
    ) -> Result<DeleteOneTokenResponse, ApiError> {
        init!(auth_backend);
        Ok(
            match find_token_with_user(&mut auth_backend, user_id, &tokenstr) {
                Ok(Some(tk)) => Delete::delete(&mut auth_backend, tk.id),
                Ok(None) => DeleteOneTokenResponse::TheSpecifiedResourceWasNotFound,
                Err(e) => e.as_backend_error_resp(),
            },
        )
    }

    async fn delete_one_user(
        &self,
        user_id: i32,
        _context: &C,
    ) -> Result<DeleteOneUserResponse, ApiError> {
        delete_all!(auth_backend, filter = user_id)
    }

    async fn get_access_rights(
        &self,
        user_id: i32,
        tokenstr: String,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetAccessRightsResponse, ApiError> {
        init!(auth_backend, offset, limit);
        Ok(
            match find_token_with_user(&mut auth_backend, user_id, &tokenstr) {
                Ok(Some(tk)) => {
                    GetAccessRightsResponse::search(&mut auth_backend, Some(tk.id), offset, limit)
                }
                Ok(None) => GetAccessRightsResponse::SuccessfulOperation(vec![]),
                Err(e) => e.as_backend_error_resp(),
            },
        )
    }

    async fn get_all_tokens(
        &self,
        tokenstr_contains: Option<&Vec<String>>,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetAllTokensResponse, ApiError> {
        search!(auth_backend, filter = tokenstr_contains)
    }

    async fn get_one_access_right(
        &self,
        user_id: i32,
        tokenstr: String,
        access_right_path: String,
        _context: &C,
    ) -> Result<GetOneAccessRightResponse, ApiError> {
        init!(auth_backend);
        Ok(
            match find_token_with_user(&mut auth_backend, user_id, &tokenstr) {
                Ok(Some(tk)) => Get::get(&mut auth_backend, (tk.id, access_right_path)),
                Ok(None) => GetOneAccessRightResponse::not_found(),
                Err(e) => e.as_backend_error_resp(),
            },
        )
    }

    async fn get_user_tokens(
        &self,
        user_id: i32,
        tokenstr_contains: Option<&Vec<String>>,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetUserTokensResponse, ApiError> {
        search!(auth_backend, filter = (user_id, tokenstr_contains))
    }

    async fn post_access_right(
        &self,
        user_id: i32,
        tokenstr: String,
        access_right: models::AccessRight,
        _context: &C,
    ) -> Result<PostAccessRightResponse, ApiError> {
        init!(auth_backend);
        Ok(
            match find_token_with_user(&mut auth_backend, user_id, &tokenstr) {
                Ok(Some(tk)) => {
                    PostAccessRightResponse::upsert(&mut auth_backend, (tk.id, access_right))
                }
                Ok(None) => PostAccessRightResponse::not_found(),
                Err(e) => e.as_backend_error_resp(),
            },
        )
    }

    async fn post_token(
        &self,
        insert: models::Token,
        _context: &C,
    ) -> Result<PostTokenResponse, ApiError> {
        post!(auth_backend)
    }

    async fn post_user_token(
        &self,
        user_id: i32,
        user_token: models::UserToken,
        _context: &C,
    ) -> Result<PostUserTokenResponse, ApiError> {
        post!(auth_backend, insert = (user_id, user_token.tokenstr))
    }

    /// Removes a specific cultivar from the collection bi its ID
    async fn delete_one_cultivar(
        &self,
        id: i32,
        _context: &C,
    ) -> Result<DeleteOneCultivarResponse, ApiError> {
        delete!(vivc_backend)
    }

    /// Returns a specific cultivar from the collection by its ID
    async fn get_one_cultivar(
        &self,
        id: i32,
        _context: &C,
    ) -> Result<GetOneCultivarResponse, ApiError> {
        get!(vivc_backend)
    }

    /// Returns a list of all stored cultivars
    async fn get_all_cultivars(
        &self,
        color: Option<&Vec<String>>,
        region: Option<&Vec<String>>,
        species: Option<&Vec<String>>,
        prime_name: Option<&Vec<String>>,
        cultivar_name: Option<&Vec<String>>,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetAllCultivarsResponse, ApiError> {
        let filter = CultivarFilter::default()
            .with_color(color)
            .with_region(region)
            .with_species(species)
            .with_prime_name(prime_name)
            .with_cultivar_name(cultivar_name);
        search!(vivc_backend)
    }

    /// Patches a specific cultivar
    async fn patch_one_cultivar(
        &self,
        id: i32,
        patch: models::CultivarPatch,
        _context: &C,
    ) -> Result<PatchOneCultivarResponse, ApiError> {
        patch!(vivc_backend)
    }

    /// Adds a new Cultivar to the collection
    async fn post_cultivar(
        &self,
        insert: models::Cultivar,
        _context: &C,
    ) -> Result<PostCultivarResponse, ApiError> {
        post!(vivc_backend)
    }

    async fn delete_one_weather_station(
        &self,
        weather_station_id: i32,
        _context: &C,
    ) -> Result<DeleteOneWeatherStationResponse, ApiError> {
        delete!(relational_backend, id = weather_station_id)
    }

    async fn get_all_weather_stations(
        &self,
        bundesland: Option<&Vec<String>>,
        name: Option<&Vec<String>>,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetAllWeatherStationsResponse, ApiError> {
        let station_filter = WeatherStationFilter::new()
            .with_bundesland_filter(bundesland)
            .with_name_filter(name);

        search!(relational_backend, filter = (station_filter, box2d))
    }

    async fn get_one_weather_station(
        &self,
        weather_station_id: i32,
        _context: &C,
    ) -> Result<GetOneWeatherStationResponse, ApiError> {
        get!(relational_backend, id = weather_station_id)
    }

    async fn patch_one_weather_station(
        &self,
        id: i32,
        patch: models::WeatherStationPatch,
        _context: &C,
    ) -> Result<PatchOneWeatherStationResponse, ApiError> {
        patch!(relational_backend)
    }

    async fn post_weather_station(
        &self,
        insert: models::WeatherStation,
        _context: &C,
    ) -> Result<PostWeatherStationResponse, ApiError> {
        post!(relational_backend)
    }

    async fn delete_one_weather_data(
        &self,
        weather_station_id: i32,
        measurement_date_str: String,
        _context: &C,
    ) -> Result<DeleteOneWeatherDataResponse, ApiError> {
        let date = parse!(NaiveDate, measurement_date_str);
        delete!(relational_backend, id = (weather_station_id, date))
    }

    async fn get_all_weather_data(
        &self,
        station_ids: Option<&Vec<i32>>,
        start_date: Option<String>,
        end_date: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        _context: &C,
    ) -> Result<GetAllWeatherDataResponse, ApiError> {
        macro_rules! opt_parse {
            ($ty:path, $e:expr) => {
                match $e {
                    Some(x) => match <$ty>::from_json(x) {
                        Ok(v) => Some(v),
                        Err(msg) => return Ok(msg.as_bad_request()),
                    },
                    None => None,
                }
            };
        }
        let start_date = opt_parse!(NaiveDate, start_date);
        let end_date = opt_parse!(NaiveDate, end_date);
        let filter = WeatherDataFilter {
            station_ids,
            start_date,
            end_date,
        };
        search!(relational_backend)
    }

    async fn get_one_weather_data(
        &self,
        weather_station_id: i32,
        measurement_date_str: String,
        _context: &C,
    ) -> Result<GetOneWeatherDataResponse, ApiError> {
        let date = parse!(NaiveDate, measurement_date_str);
        get!(relational_backend, id = (weather_station_id, date))
    }

    async fn patch_one_weather_data(
        &self,
        weather_station_id: i32,
        measurement_date_str: String,
        patch: models::WeatherDataPatch,
        _context: &C,
    ) -> Result<PatchOneWeatherDataResponse, ApiError> {
        let date = parse!(NaiveDate, measurement_date_str);
        patch!(relational_backend, id = (weather_station_id, date))
    }

    async fn post_weather_data(
        &self,
        weather_data: models::WeatherData,
        _context: &C,
    ) -> Result<PostWeatherDataResponse, ApiError> {
        init!(relational_backend);
        let insert = parse!(WeatherData, weather_data);
        Ok(match insert.create(&mut relational_backend) {
            Ok((station_id, date)) => (station_id, date_to_str(date)).into(),
            Err(e) => RelationalBackendLibError::from(e).as_backend_error_resp(),
        })
    }
}
