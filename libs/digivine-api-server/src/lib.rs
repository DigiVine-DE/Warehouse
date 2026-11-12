#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_extern_crates,
    non_camel_case_types,
    clippy::all
)]

use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    task::{Context, Poll},
};
use swagger::{ApiError, ContextWrapper};

pub trait AsBackendErrorResp<T> {
    fn default_code() -> &'static str;
    fn as_backend_error_resp_with_code(&self, code: String) -> T;

    fn as_backend_error_resp(&self) -> T {
        self.as_backend_error_resp_with_code(Self::default_code().to_string())
    }
}

pub trait AsBadRequestResponse<T> {
    fn as_bad_request(self) -> T;
}

pub trait NotFoundResponse {
    fn not_found() -> Self;
}

pub trait UnitSuccessResponse {
    fn succeed() -> Self;
}

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &'static str = "/api/v0.3";
pub const API_VERSION: &'static str = "0.3";

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOneCultivarResponse {
    /// Successfully deleted cultivar
    SuccessfullyDeletedCultivar,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAllCultivarsResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::Cultivar>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetOneCultivarResponse {
    /// Successful operation
    SuccessfulOperation(models::Cultivar),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PatchOneCultivarResponse {
    /// Successful operation
    SuccessfulOperation(models::Cultivar),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostCultivarResponse {
    /// Successful operation
    SuccessfulOperation(models::InlineResponse2017),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteFieldResponse {
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Method not allowed
    MethodNotAllowed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOneFieldResponse {
    /// Successfully deleted Field
    SuccessfullyDeletedField,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAllFieldsResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::Field>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetOneFieldResponse {
    /// Successful operation
    SuccessfulOperation(models::Field),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PatchOneFieldResponse {
    /// Successful operation
    SuccessfulOperation(models::Field),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostFieldResponse {
    /// Successful operation
    SuccessfulOperation(models::InlineResponse2011),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteForecastResponse {
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Method not allowed
    MethodNotAllowed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOneForecastResponse {
    /// Successfully deleted Forecast
    SuccessfullyDeletedForecast,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAllForecastsResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::Forecast>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetOneForecastResponse {
    /// Successful operation
    SuccessfulOperation(models::Forecast),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostForecastResponse {
    /// Successful operation
    SuccessfulOperation(models::InlineResponse201),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteCompletedHarvestTaskResponse {
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Method not allowed
    MethodNotAllowed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOneCompletedHarvestTaskResponse {
    /// Successfully deleted completed Harvest
    SuccessfullyDeletedCompletedHarvest,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOneScheduledHarvestTaskResponse {
    /// Successfully deleted scheduled Harvest Task
    SuccessfullyDeletedScheduledHarvestTask,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteScheduledHarvestTaskResponse {
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Method not allowed
    MethodNotAllowed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAllCompletedHarvestTasksResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::HarvestOrTask>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAllScheduledHarvestTasksResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::HarvestTask>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetOneCompletedHarvestTaskResponse {
    /// Successful operation
    SuccessfulOperation(models::HarvestOrTask),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetOneScheduledHarvestTaskResponse {
    /// Successful operation
    SuccessfulOperation(models::HarvestTask),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostCompletedHarvestTaskResponse {
    /// Successful operation
    SuccessfulOperation(models::InlineResponse2015),
    /// See Other
    SeeOther { location: Option<String> },
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostScheduledHarvestTaskResponse {
    /// Successful operation
    SuccessfulOperation(models::InlineResponse2015),
    /// See Other
    SeeOther { location: Option<String> },
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOnePlantResponse {
    /// Successfully deleted Plant
    SuccessfullyDeletedPlant,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeletePlantResponse {
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Method not allowed
    MethodNotAllowed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAllPlantsResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::Plant>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetOnePlantResponse {
    /// Successful operation
    SuccessfulOperation(models::Plant),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PatchOnePlantResponse {
    /// Successful operation
    SuccessfulOperation(models::Plant),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostPlantResponse {
    /// Successful operation
    SuccessfulOperation(models::InlineResponse2013),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOnePlanterRunResponse {
    /// Successfully deleted Planter Run
    SuccessfullyDeletedPlanterRun,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeletePlanterRunResponse {
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Method not allowed
    MethodNotAllowed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAllPlanterRunsResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::PlanterRun>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetOnePlanterRunResponse {
    /// Successful operation
    SuccessfulOperation(models::PlanterRun),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostPlanterRunResponse {
    /// Successful operation
    SuccessfulOperation(models::InlineResponse2012),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOneQualityMeasurementResponse {
    /// Successfully deleted Quality Measurement
    SuccessfullyDeletedQualityMeasurement,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteQualityMeasurementResponse {
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Method not allowed
    MethodNotAllowed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAllQualityMeasurementsResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::QualityMeasurement>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetOneQualityMeasurementResponse {
    /// Successful operation
    SuccessfulOperation(models::QualityMeasurement),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostQualityMeasurementResponse {
    /// Successful operation
    SuccessfulOperation(models::InlineResponse2014),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteCompletedSprayTaskResponse {
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Method not allowed
    MethodNotAllowed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOneCompletedSprayTaskResponse {
    /// Successfully deleted completed Spray
    SuccessfullyDeletedCompletedSpray,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOneScheduledSprayTaskResponse {
    /// Successfully deleted scheduled Spray Task
    SuccessfullyDeletedScheduledSprayTask,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteScheduledSprayTaskResponse {
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Method not allowed
    MethodNotAllowed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAllCompletedSprayTasksResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::SprayTask>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAllScheduledSprayTasksResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::SprayTask>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetOneCompletedSprayTaskResponse {
    /// Successful operation
    SuccessfulOperation(models::SprayTask),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetOneScheduledSprayTaskResponse {
    /// Successful operation
    SuccessfulOperation(models::SprayTask),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostCompletedSprayTaskResponse {
    /// Successful operation
    SuccessfulOperation(models::InlineResponse2016),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostScheduledSprayTaskResponse {
    /// Successful operation
    SuccessfulOperation(models::InlineResponse2016),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOneTerrainResponse {
    /// Successfully deleted Terrain Model
    SuccessfullyDeletedTerrainModel,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteTerrainResponse {
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Method not allowed
    MethodNotAllowed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAllTerrainsResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::Terrain>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetOneTerrainResponse {
    /// Successful operation
    SuccessfulOperation(models::Terrain),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOneAccessRightResponse {
    /// Successfully deleted access right
    SuccessfullyDeletedAccessRight,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOneTokenResponse {
    /// Successfully deleted Token
    SuccessfullyDeletedToken,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOneUserResponse {
    /// Successfully deleted user
    SuccessfullyDeletedUser(Vec<models::Token>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAccessRightsResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::AccessRight>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAllTokensResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::Token>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetOneAccessRightResponse {
    /// Successful operation
    SuccessfulOperation(models::AccessRight),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetUserTokensResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::Token>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostAccessRightResponse {
    /// Successful operation
    SuccessfulOperation(models::AccessRight),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostTokenResponse {
    /// Successful operation
    SuccessfulOperation(models::TokenId),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostUserTokenResponse {
    /// Successful operation
    SuccessfulOperation(models::TokenId),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOneWeatherDataResponse {
    /// Successfully deleted weather data
    SuccessfullyDeletedWeatherData,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAllWeatherDataResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::WeatherData>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetOneWeatherDataResponse {
    /// Successful operation
    SuccessfulOperation(models::WeatherData),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PatchOneWeatherDataResponse {
    /// Successful operation
    SuccessfulOperation(models::WeatherData),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostWeatherDataResponse {
    /// Successful operation
    SuccessfulOperation(models::InlineResponse2019),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum DeleteOneWeatherStationResponse {
    /// Successfully deleted weather station
    SuccessfullyDeletedWeatherStation,
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetAllWeatherStationsResponse {
    /// Successful operation
    SuccessfulOperation(Vec<models::WeatherStation>),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum GetOneWeatherStationResponse {
    /// Successful operation
    SuccessfulOperation(models::WeatherStation),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PatchOneWeatherStationResponse {
    /// Successful operation
    SuccessfulOperation(models::WeatherStation),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// The specified resource was not found
    TheSpecifiedResourceWasNotFound,
    /// Backend Error
    BackendError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, macros_lib::Response)]
#[must_use]
pub enum PostWeatherStationResponse {
    /// Successful operation
    SuccessfulOperation(models::InlineResponse2018),
    /// Bad Request: Could not parse parameters
    BadRequest(models::InlineResponse400),
    /// Missing API key.
    MissingAPIKey,
    /// Specified API key does not have the required rights.
    SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
    /// Backend Error
    BackendError(models::Error),
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(
        &self,
        _cx: &mut Context,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Removes a specific cultivar from the collection bi its ID
    async fn delete_one_cultivar(
        &self,
        cultivar_id: i32,
        context: &C,
    ) -> Result<DeleteOneCultivarResponse, ApiError>;

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
        context: &C,
    ) -> Result<GetAllCultivarsResponse, ApiError>;

    /// Returns a specific cultivar from the collection by its ID
    async fn get_one_cultivar(
        &self,
        cultivar_id: i32,
        context: &C,
    ) -> Result<GetOneCultivarResponse, ApiError>;

    /// Patches a specific cultivar
    async fn patch_one_cultivar(
        &self,
        cultivar_id: i32,
        cultivar_patch: models::CultivarPatch,
        context: &C,
    ) -> Result<PatchOneCultivarResponse, ApiError>;

    /// Adds a new Cultivar to the collection
    async fn post_cultivar(
        &self,
        cultivar: models::Cultivar,
        context: &C,
    ) -> Result<PostCultivarResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_field(&self, context: &C) -> Result<DeleteFieldResponse, ApiError>;

    /// Removes a specific stored Field from the collection
    async fn delete_one_field(
        &self,
        field_id: i64,
        context: &C,
    ) -> Result<DeleteOneFieldResponse, ApiError>;

    /// Returns a list of (all) stored Fields
    async fn get_all_fields(
        &self,
        box2d: Option<&Vec<f64>>,
        fuzzy_field_keywords: Option<&Vec<String>>,
        field_name_contains: Option<&Vec<String>>,
        fruit_name_contains: Option<&Vec<String>>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllFieldsResponse, ApiError>;

    /// Returns a specific stored Field from the collection
    async fn get_one_field(
        &self,
        field_id: i64,
        context: &C,
    ) -> Result<GetOneFieldResponse, ApiError>;

    /// Patches a specific stored Field
    async fn patch_one_field(
        &self,
        field_id: i64,
        field_patch: models::FieldPatch,
        context: &C,
    ) -> Result<PatchOneFieldResponse, ApiError>;

    /// Adds a new Field to the collection
    async fn post_field(
        &self,
        field: models::Field,
        context: &C,
    ) -> Result<PostFieldResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_forecast(&self, context: &C) -> Result<DeleteForecastResponse, ApiError>;

    /// Removes a specific stored Forecast from the collection
    async fn delete_one_forecast(
        &self,
        forecast_id: i64,
        context: &C,
    ) -> Result<DeleteOneForecastResponse, ApiError>;

    /// Returns a list of (all) stored Forecasts
    async fn get_all_forecasts(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllForecastsResponse, ApiError>;

    /// Returns a specific stored Forecast from the collection
    async fn get_one_forecast(
        &self,
        forecast_id: i64,
        context: &C,
    ) -> Result<GetOneForecastResponse, ApiError>;

    /// Adds a new Forecast to the collection
    async fn post_forecast(
        &self,
        forecast: models::Forecast,
        context: &C,
    ) -> Result<PostForecastResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_completed_harvest_task(
        &self,
        context: &C,
    ) -> Result<DeleteCompletedHarvestTaskResponse, ApiError>;

    /// Removes either a specific stored Harvest Task or a specific stored Harvest with Harvest Measurements from the completed collection
    async fn delete_one_completed_harvest_task(
        &self,
        harvest_task_id: i64,
        context: &C,
    ) -> Result<DeleteOneCompletedHarvestTaskResponse, ApiError>;

    /// Removes a specific stored Harvest Task from the scheduled collection
    async fn delete_one_scheduled_harvest_task(
        &self,
        harvest_task_id: i64,
        context: &C,
    ) -> Result<DeleteOneScheduledHarvestTaskResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_scheduled_harvest_task(
        &self,
        context: &C,
    ) -> Result<DeleteScheduledHarvestTaskResponse, ApiError>;

    /// Returns a list of (all) stored completed Harvest Tasks and Harvests with Harvest Measurements
    async fn get_all_completed_harvest_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllCompletedHarvestTasksResponse, ApiError>;

    /// Returns a list of (all) stored scheduled Harvest Tasks
    async fn get_all_scheduled_harvest_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllScheduledHarvestTasksResponse, ApiError>;

    /// Returns either a specific stored Harvest Task or a specific stored Harvest with Harvest Measurements from the completed collection
    async fn get_one_completed_harvest_task(
        &self,
        harvest_task_id: i64,
        context: &C,
    ) -> Result<GetOneCompletedHarvestTaskResponse, ApiError>;

    /// Returns a specific stored Harvest Task from the scheduled collection
    async fn get_one_scheduled_harvest_task(
        &self,
        harvest_task_id: i64,
        context: &C,
    ) -> Result<GetOneScheduledHarvestTaskResponse, ApiError>;

    /// Adds either a new Harvest Task or a new Harvest with Harvest Measurements to the completed collection
    async fn post_completed_harvest_task(
        &self,
        harvest_or_task: models::HarvestOrTask,
        context: &C,
    ) -> Result<PostCompletedHarvestTaskResponse, ApiError>;

    /// Adds a new Harvest Task to the scheduled collection
    async fn post_scheduled_harvest_task(
        &self,
        harvest_task: models::HarvestTask,
        context: &C,
    ) -> Result<PostScheduledHarvestTaskResponse, ApiError>;

    /// Removes a specific stored Plant from the collection
    async fn delete_one_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        context: &C,
    ) -> Result<DeleteOnePlantResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
        context: &C,
    ) -> Result<DeletePlantResponse, ApiError>;

    /// Returns a list of (all) stored Plants
    async fn get_all_plants(
        &self,
        field_id: i64,
        planter_run_id: i64,
        offset: Option<i32>,
        limit: Option<i32>,
        box2d: Option<&Vec<f64>>,
        context: &C,
    ) -> Result<GetAllPlantsResponse, ApiError>;

    /// Returns a specific stored Plant from the collection
    async fn get_one_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        context: &C,
    ) -> Result<GetOnePlantResponse, ApiError>;

    /// Patches a specific stored Plant
    async fn patch_one_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        plant_patch: models::PlantPatch,
        context: &C,
    ) -> Result<PatchOnePlantResponse, ApiError>;

    /// Adds a new Plant to the collection
    async fn post_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant: models::Plant,
        context: &C,
    ) -> Result<PostPlantResponse, ApiError>;

    /// Removes a specific stored Planter Run from the collection
    async fn delete_one_planter_run(
        &self,
        field_id: i64,
        planter_run_id: i64,
        context: &C,
    ) -> Result<DeleteOnePlanterRunResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_planter_run(
        &self,
        field_id: i64,
        context: &C,
    ) -> Result<DeletePlanterRunResponse, ApiError>;

    /// Returns a list of (all) stored Planter Runs
    async fn get_all_planter_runs(
        &self,
        field_id: i64,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllPlanterRunsResponse, ApiError>;

    /// Returns a specific stored Planter Run from the collection
    async fn get_one_planter_run(
        &self,
        field_id: i64,
        planter_run_id: i64,
        context: &C,
    ) -> Result<GetOnePlanterRunResponse, ApiError>;

    /// Adds a new Planter Run to the collection
    async fn post_planter_run(
        &self,
        field_id: i64,
        planter_run: models::PlanterRun,
        context: &C,
    ) -> Result<PostPlanterRunResponse, ApiError>;

    /// Removes a specific stored Quality Measurement from the collection
    async fn delete_one_quality_measurement(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        quality_measurement_id: i64,
        context: &C,
    ) -> Result<DeleteOneQualityMeasurementResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_quality_measurement(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        context: &C,
    ) -> Result<DeleteQualityMeasurementResponse, ApiError>;

    /// Returns a list of (all) stored Quality Measurements
    async fn get_all_quality_measurements(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllQualityMeasurementsResponse, ApiError>;

    /// Returns a specific stored Quality Measurement from the collection
    async fn get_one_quality_measurement(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        quality_measurement_id: i64,
        context: &C,
    ) -> Result<GetOneQualityMeasurementResponse, ApiError>;

    /// Adds a new Quality Measurement to the collection
    async fn post_quality_measurement(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        quality_measurement: models::QualityMeasurement,
        context: &C,
    ) -> Result<PostQualityMeasurementResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_completed_spray_task(
        &self,
        context: &C,
    ) -> Result<DeleteCompletedSprayTaskResponse, ApiError>;

    /// Removes either a specific stored Spray Task or a specific stored Spray with Spray Measurements from the completed collection
    async fn delete_one_completed_spray_task(
        &self,
        spray_task_id: i64,
        context: &C,
    ) -> Result<DeleteOneCompletedSprayTaskResponse, ApiError>;

    /// Removes a specific stored Spray Task from the scheduled collection
    async fn delete_one_scheduled_spray_task(
        &self,
        spray_task_id: i64,
        context: &C,
    ) -> Result<DeleteOneScheduledSprayTaskResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_scheduled_spray_task(
        &self,
        context: &C,
    ) -> Result<DeleteScheduledSprayTaskResponse, ApiError>;

    /// Returns a list of (all) stored completed Spray Tasks and Sprays with Spray Measurements
    async fn get_all_completed_spray_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllCompletedSprayTasksResponse, ApiError>;

    /// Returns a list of (all) stored scheduled Spray Tasks
    async fn get_all_scheduled_spray_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllScheduledSprayTasksResponse, ApiError>;

    /// Returns either a specific stored Spray Task or a specific stored Spray with Spray Measurements from the completed collection
    async fn get_one_completed_spray_task(
        &self,
        spray_task_id: i64,
        context: &C,
    ) -> Result<GetOneCompletedSprayTaskResponse, ApiError>;

    /// Returns a specific stored Spray Task from the scheduled collection
    async fn get_one_scheduled_spray_task(
        &self,
        spray_task_id: i64,
        context: &C,
    ) -> Result<GetOneScheduledSprayTaskResponse, ApiError>;

    /// Adds either a new Spray Task or a new Spray with Spray Measurements to the completed collection
    async fn post_completed_spray_task(
        &self,
        spray_task: models::SprayTask,
        context: &C,
    ) -> Result<PostCompletedSprayTaskResponse, ApiError>;

    /// Adds a new Spray Task to the scheduled collection
    async fn post_scheduled_spray_task(
        &self,
        spray_task: models::SprayTask,
        context: &C,
    ) -> Result<PostScheduledSprayTaskResponse, ApiError>;

    /// Removes a specific stored Terrain Model from the collection
    async fn delete_one_terrain(
        &self,
        terrain_id: i64,
        context: &C,
    ) -> Result<DeleteOneTerrainResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_terrain(&self, context: &C) -> Result<DeleteTerrainResponse, ApiError>;

    /// Returns a list of (all) stored Terrain Models
    async fn get_all_terrains(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllTerrainsResponse, ApiError>;

    /// Returns a specific stored Terrain Model from the collection
    async fn get_one_terrain(
        &self,
        terrain_id: i64,
        context: &C,
    ) -> Result<GetOneTerrainResponse, ApiError>;

    /// Deletes a specific access right for a specific token
    async fn delete_one_access_right(
        &self,
        user_id: i32,
        tokenstr: String,
        access_right_path: String,
        context: &C,
    ) -> Result<DeleteOneAccessRightResponse, ApiError>;

    /// Deletes a specific token
    async fn delete_one_token(
        &self,
        user_id: i32,
        tokenstr: String,
        context: &C,
    ) -> Result<DeleteOneTokenResponse, ApiError>;

    /// Deletes the user and all its tokens
    async fn delete_one_user(
        &self,
        user_id: i32,
        context: &C,
    ) -> Result<DeleteOneUserResponse, ApiError>;

    /// Returns a list of access rights for a specific token
    async fn get_access_rights(
        &self,
        user_id: i32,
        tokenstr: String,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C,
    ) -> Result<GetAccessRightsResponse, ApiError>;

    /// Returns a list of all existing tokens
    async fn get_all_tokens(
        &self,
        tokenstr_contains: Option<&Vec<String>>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllTokensResponse, ApiError>;

    /// Returns information about a specific access right for a specific token
    async fn get_one_access_right(
        &self,
        user_id: i32,
        tokenstr: String,
        access_right_path: String,
        context: &C,
    ) -> Result<GetOneAccessRightResponse, ApiError>;

    /// Returns all tokens of the user
    async fn get_user_tokens(
        &self,
        user_id: i32,
        tokenstr_contains: Option<&Vec<String>>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C,
    ) -> Result<GetUserTokensResponse, ApiError>;

    /// Creates a new access right for a specific token
    async fn post_access_right(
        &self,
        user_id: i32,
        tokenstr: String,
        access_right: models::AccessRight,
        context: &C,
    ) -> Result<PostAccessRightResponse, ApiError>;

    /// Creates a new token
    async fn post_token(
        &self,
        token: models::Token,
        context: &C,
    ) -> Result<PostTokenResponse, ApiError>;

    /// Creates a new token of the given `tokenstr`
    async fn post_user_token(
        &self,
        user_id: i32,
        user_token: models::UserToken,
        context: &C,
    ) -> Result<PostUserTokenResponse, ApiError>;

    /// Removes weather data for a specific station and date
    async fn delete_one_weather_data(
        &self,
        weather_station_id: i32,
        measurement_date: String,
        context: &C,
    ) -> Result<DeleteOneWeatherDataResponse, ApiError>;

    /// Returns a list of weather data
    async fn get_all_weather_data(
        &self,
        station_ids: Option<&Vec<i32>>,
        start_date: Option<String>,
        end_date: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllWeatherDataResponse, ApiError>;

    /// Returns weather data for a specific station and date
    async fn get_one_weather_data(
        &self,
        weather_station_id: i32,
        measurement_date: String,
        context: &C,
    ) -> Result<GetOneWeatherDataResponse, ApiError>;

    /// Patches weather data for a specific station and date
    async fn patch_one_weather_data(
        &self,
        weather_station_id: i32,
        measurement_date: String,
        weather_data_patch: models::WeatherDataPatch,
        context: &C,
    ) -> Result<PatchOneWeatherDataResponse, ApiError>;

    /// Adds new weather data
    async fn post_weather_data(
        &self,
        weather_data: models::WeatherData,
        context: &C,
    ) -> Result<PostWeatherDataResponse, ApiError>;

    /// Removes a specific weather station from the collection
    async fn delete_one_weather_station(
        &self,
        weather_station_id: i32,
        context: &C,
    ) -> Result<DeleteOneWeatherStationResponse, ApiError>;

    /// Returns a list of all weather stations
    async fn get_all_weather_stations(
        &self,
        bundesland: Option<&Vec<String>>,
        name: Option<&Vec<String>>,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllWeatherStationsResponse, ApiError>;

    /// Returns a specific weather station from the collection
    async fn get_one_weather_station(
        &self,
        weather_station_id: i32,
        context: &C,
    ) -> Result<GetOneWeatherStationResponse, ApiError>;

    /// Patches a specific weather station
    async fn patch_one_weather_station(
        &self,
        weather_station_id: i32,
        weather_station_patch: models::WeatherStationPatch,
        context: &C,
    ) -> Result<PatchOneWeatherStationResponse, ApiError>;

    /// Adds a new Weather Station to the collection
    async fn post_weather_station(
        &self,
        weather_station: models::WeatherStation,
        context: &C,
    ) -> Result<PostWeatherStationResponse, ApiError>;
}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {
    fn poll_ready(
        &self,
        _cx: &mut Context,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Removes a specific cultivar from the collection bi its ID
    async fn delete_one_cultivar(
        &self,
        cultivar_id: i32,
    ) -> Result<DeleteOneCultivarResponse, ApiError>;

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
    ) -> Result<GetAllCultivarsResponse, ApiError>;

    /// Returns a specific cultivar from the collection by its ID
    async fn get_one_cultivar(&self, cultivar_id: i32) -> Result<GetOneCultivarResponse, ApiError>;

    /// Patches a specific cultivar
    async fn patch_one_cultivar(
        &self,
        cultivar_id: i32,
        cultivar_patch: models::CultivarPatch,
    ) -> Result<PatchOneCultivarResponse, ApiError>;

    /// Adds a new Cultivar to the collection
    async fn post_cultivar(
        &self,
        cultivar: models::Cultivar,
    ) -> Result<PostCultivarResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_field(&self) -> Result<DeleteFieldResponse, ApiError>;

    /// Removes a specific stored Field from the collection
    async fn delete_one_field(&self, field_id: i64) -> Result<DeleteOneFieldResponse, ApiError>;

    /// Returns a list of (all) stored Fields
    async fn get_all_fields(
        &self,
        box2d: Option<&Vec<f64>>,
        fuzzy_field_keywords: Option<&Vec<String>>,
        field_name_contains: Option<&Vec<String>>,
        fruit_name_contains: Option<&Vec<String>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllFieldsResponse, ApiError>;

    /// Returns a specific stored Field from the collection
    async fn get_one_field(&self, field_id: i64) -> Result<GetOneFieldResponse, ApiError>;

    /// Patches a specific stored Field
    async fn patch_one_field(
        &self,
        field_id: i64,
        field_patch: models::FieldPatch,
    ) -> Result<PatchOneFieldResponse, ApiError>;

    /// Adds a new Field to the collection
    async fn post_field(&self, field: models::Field) -> Result<PostFieldResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_forecast(&self) -> Result<DeleteForecastResponse, ApiError>;

    /// Removes a specific stored Forecast from the collection
    async fn delete_one_forecast(
        &self,
        forecast_id: i64,
    ) -> Result<DeleteOneForecastResponse, ApiError>;

    /// Returns a list of (all) stored Forecasts
    async fn get_all_forecasts(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllForecastsResponse, ApiError>;

    /// Returns a specific stored Forecast from the collection
    async fn get_one_forecast(&self, forecast_id: i64) -> Result<GetOneForecastResponse, ApiError>;

    /// Adds a new Forecast to the collection
    async fn post_forecast(
        &self,
        forecast: models::Forecast,
    ) -> Result<PostForecastResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_completed_harvest_task(
        &self,
    ) -> Result<DeleteCompletedHarvestTaskResponse, ApiError>;

    /// Removes either a specific stored Harvest Task or a specific stored Harvest with Harvest Measurements from the completed collection
    async fn delete_one_completed_harvest_task(
        &self,
        harvest_task_id: i64,
    ) -> Result<DeleteOneCompletedHarvestTaskResponse, ApiError>;

    /// Removes a specific stored Harvest Task from the scheduled collection
    async fn delete_one_scheduled_harvest_task(
        &self,
        harvest_task_id: i64,
    ) -> Result<DeleteOneScheduledHarvestTaskResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_scheduled_harvest_task(
        &self,
    ) -> Result<DeleteScheduledHarvestTaskResponse, ApiError>;

    /// Returns a list of (all) stored completed Harvest Tasks and Harvests with Harvest Measurements
    async fn get_all_completed_harvest_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllCompletedHarvestTasksResponse, ApiError>;

    /// Returns a list of (all) stored scheduled Harvest Tasks
    async fn get_all_scheduled_harvest_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllScheduledHarvestTasksResponse, ApiError>;

    /// Returns either a specific stored Harvest Task or a specific stored Harvest with Harvest Measurements from the completed collection
    async fn get_one_completed_harvest_task(
        &self,
        harvest_task_id: i64,
    ) -> Result<GetOneCompletedHarvestTaskResponse, ApiError>;

    /// Returns a specific stored Harvest Task from the scheduled collection
    async fn get_one_scheduled_harvest_task(
        &self,
        harvest_task_id: i64,
    ) -> Result<GetOneScheduledHarvestTaskResponse, ApiError>;

    /// Adds either a new Harvest Task or a new Harvest with Harvest Measurements to the completed collection
    async fn post_completed_harvest_task(
        &self,
        harvest_or_task: models::HarvestOrTask,
    ) -> Result<PostCompletedHarvestTaskResponse, ApiError>;

    /// Adds a new Harvest Task to the scheduled collection
    async fn post_scheduled_harvest_task(
        &self,
        harvest_task: models::HarvestTask,
    ) -> Result<PostScheduledHarvestTaskResponse, ApiError>;

    /// Removes a specific stored Plant from the collection
    async fn delete_one_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
    ) -> Result<DeleteOnePlantResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
    ) -> Result<DeletePlantResponse, ApiError>;

    /// Returns a list of (all) stored Plants
    async fn get_all_plants(
        &self,
        field_id: i64,
        planter_run_id: i64,
        offset: Option<i32>,
        limit: Option<i32>,
        box2d: Option<&Vec<f64>>,
    ) -> Result<GetAllPlantsResponse, ApiError>;

    /// Returns a specific stored Plant from the collection
    async fn get_one_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
    ) -> Result<GetOnePlantResponse, ApiError>;

    /// Patches a specific stored Plant
    async fn patch_one_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        plant_patch: models::PlantPatch,
    ) -> Result<PatchOnePlantResponse, ApiError>;

    /// Adds a new Plant to the collection
    async fn post_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant: models::Plant,
    ) -> Result<PostPlantResponse, ApiError>;

    /// Removes a specific stored Planter Run from the collection
    async fn delete_one_planter_run(
        &self,
        field_id: i64,
        planter_run_id: i64,
    ) -> Result<DeleteOnePlanterRunResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_planter_run(&self, field_id: i64)
        -> Result<DeletePlanterRunResponse, ApiError>;

    /// Returns a list of (all) stored Planter Runs
    async fn get_all_planter_runs(
        &self,
        field_id: i64,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllPlanterRunsResponse, ApiError>;

    /// Returns a specific stored Planter Run from the collection
    async fn get_one_planter_run(
        &self,
        field_id: i64,
        planter_run_id: i64,
    ) -> Result<GetOnePlanterRunResponse, ApiError>;

    /// Adds a new Planter Run to the collection
    async fn post_planter_run(
        &self,
        field_id: i64,
        planter_run: models::PlanterRun,
    ) -> Result<PostPlanterRunResponse, ApiError>;

    /// Removes a specific stored Quality Measurement from the collection
    async fn delete_one_quality_measurement(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        quality_measurement_id: i64,
    ) -> Result<DeleteOneQualityMeasurementResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_quality_measurement(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
    ) -> Result<DeleteQualityMeasurementResponse, ApiError>;

    /// Returns a list of (all) stored Quality Measurements
    async fn get_all_quality_measurements(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllQualityMeasurementsResponse, ApiError>;

    /// Returns a specific stored Quality Measurement from the collection
    async fn get_one_quality_measurement(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        quality_measurement_id: i64,
    ) -> Result<GetOneQualityMeasurementResponse, ApiError>;

    /// Adds a new Quality Measurement to the collection
    async fn post_quality_measurement(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        quality_measurement: models::QualityMeasurement,
    ) -> Result<PostQualityMeasurementResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_completed_spray_task(
        &self,
    ) -> Result<DeleteCompletedSprayTaskResponse, ApiError>;

    /// Removes either a specific stored Spray Task or a specific stored Spray with Spray Measurements from the completed collection
    async fn delete_one_completed_spray_task(
        &self,
        spray_task_id: i64,
    ) -> Result<DeleteOneCompletedSprayTaskResponse, ApiError>;

    /// Removes a specific stored Spray Task from the scheduled collection
    async fn delete_one_scheduled_spray_task(
        &self,
        spray_task_id: i64,
    ) -> Result<DeleteOneScheduledSprayTaskResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_scheduled_spray_task(
        &self,
    ) -> Result<DeleteScheduledSprayTaskResponse, ApiError>;

    /// Returns a list of (all) stored completed Spray Tasks and Sprays with Spray Measurements
    async fn get_all_completed_spray_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllCompletedSprayTasksResponse, ApiError>;

    /// Returns a list of (all) stored scheduled Spray Tasks
    async fn get_all_scheduled_spray_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllScheduledSprayTasksResponse, ApiError>;

    /// Returns either a specific stored Spray Task or a specific stored Spray with Spray Measurements from the completed collection
    async fn get_one_completed_spray_task(
        &self,
        spray_task_id: i64,
    ) -> Result<GetOneCompletedSprayTaskResponse, ApiError>;

    /// Returns a specific stored Spray Task from the scheduled collection
    async fn get_one_scheduled_spray_task(
        &self,
        spray_task_id: i64,
    ) -> Result<GetOneScheduledSprayTaskResponse, ApiError>;

    /// Adds either a new Spray Task or a new Spray with Spray Measurements to the completed collection
    async fn post_completed_spray_task(
        &self,
        spray_task: models::SprayTask,
    ) -> Result<PostCompletedSprayTaskResponse, ApiError>;

    /// Adds a new Spray Task to the scheduled collection
    async fn post_scheduled_spray_task(
        &self,
        spray_task: models::SprayTask,
    ) -> Result<PostScheduledSprayTaskResponse, ApiError>;

    /// Removes a specific stored Terrain Model from the collection
    async fn delete_one_terrain(
        &self,
        terrain_id: i64,
    ) -> Result<DeleteOneTerrainResponse, ApiError>;

    /// Can't delete complete collection
    async fn delete_terrain(&self) -> Result<DeleteTerrainResponse, ApiError>;

    /// Returns a list of (all) stored Terrain Models
    async fn get_all_terrains(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllTerrainsResponse, ApiError>;

    /// Returns a specific stored Terrain Model from the collection
    async fn get_one_terrain(&self, terrain_id: i64) -> Result<GetOneTerrainResponse, ApiError>;

    /// Deletes a specific access right for a specific token
    async fn delete_one_access_right(
        &self,
        user_id: i32,
        tokenstr: String,
        access_right_path: String,
    ) -> Result<DeleteOneAccessRightResponse, ApiError>;

    /// Deletes a specific token
    async fn delete_one_token(
        &self,
        user_id: i32,
        tokenstr: String,
    ) -> Result<DeleteOneTokenResponse, ApiError>;

    /// Deletes the user and all its tokens
    async fn delete_one_user(&self, user_id: i32) -> Result<DeleteOneUserResponse, ApiError>;

    /// Returns a list of access rights for a specific token
    async fn get_access_rights(
        &self,
        user_id: i32,
        tokenstr: String,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAccessRightsResponse, ApiError>;

    /// Returns a list of all existing tokens
    async fn get_all_tokens(
        &self,
        tokenstr_contains: Option<&Vec<String>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllTokensResponse, ApiError>;

    /// Returns information about a specific access right for a specific token
    async fn get_one_access_right(
        &self,
        user_id: i32,
        tokenstr: String,
        access_right_path: String,
    ) -> Result<GetOneAccessRightResponse, ApiError>;

    /// Returns all tokens of the user
    async fn get_user_tokens(
        &self,
        user_id: i32,
        tokenstr_contains: Option<&Vec<String>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetUserTokensResponse, ApiError>;

    /// Creates a new access right for a specific token
    async fn post_access_right(
        &self,
        user_id: i32,
        tokenstr: String,
        access_right: models::AccessRight,
    ) -> Result<PostAccessRightResponse, ApiError>;

    /// Creates a new token
    async fn post_token(&self, token: models::Token) -> Result<PostTokenResponse, ApiError>;

    /// Creates a new token of the given `tokenstr`
    async fn post_user_token(
        &self,
        user_id: i32,
        user_token: models::UserToken,
    ) -> Result<PostUserTokenResponse, ApiError>;

    /// Removes weather data for a specific station and date
    async fn delete_one_weather_data(
        &self,
        weather_station_id: i32,
        measurement_date: String,
    ) -> Result<DeleteOneWeatherDataResponse, ApiError>;

    /// Returns a list of weather data
    async fn get_all_weather_data(
        &self,
        station_ids: Option<&Vec<i32>>,
        start_date: Option<String>,
        end_date: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllWeatherDataResponse, ApiError>;

    /// Returns weather data for a specific station and date
    async fn get_one_weather_data(
        &self,
        weather_station_id: i32,
        measurement_date: String,
    ) -> Result<GetOneWeatherDataResponse, ApiError>;

    /// Patches weather data for a specific station and date
    async fn patch_one_weather_data(
        &self,
        weather_station_id: i32,
        measurement_date: String,
        weather_data_patch: models::WeatherDataPatch,
    ) -> Result<PatchOneWeatherDataResponse, ApiError>;

    /// Adds new weather data
    async fn post_weather_data(
        &self,
        weather_data: models::WeatherData,
    ) -> Result<PostWeatherDataResponse, ApiError>;

    /// Removes a specific weather station from the collection
    async fn delete_one_weather_station(
        &self,
        weather_station_id: i32,
    ) -> Result<DeleteOneWeatherStationResponse, ApiError>;

    /// Returns a list of all weather stations
    async fn get_all_weather_stations(
        &self,
        bundesland: Option<&Vec<String>>,
        name: Option<&Vec<String>>,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllWeatherStationsResponse, ApiError>;

    /// Returns a specific weather station from the collection
    async fn get_one_weather_station(
        &self,
        weather_station_id: i32,
    ) -> Result<GetOneWeatherStationResponse, ApiError>;

    /// Patches a specific weather station
    async fn patch_one_weather_station(
        &self,
        weather_station_id: i32,
        weather_station_patch: models::WeatherStationPatch,
    ) -> Result<PatchOneWeatherStationResponse, ApiError>;

    /// Adds a new Weather Station to the collection
    async fn post_weather_station(
        &self,
        weather_station: models::WeatherStation,
    ) -> Result<PostWeatherStationResponse, ApiError>;
}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync>
where
    Self: Sized,
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
        ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Removes a specific cultivar from the collection bi its ID
    async fn delete_one_cultivar(
        &self,
        cultivar_id: i32,
    ) -> Result<DeleteOneCultivarResponse, ApiError> {
        let context = self.context().clone();
        self.api().delete_one_cultivar(cultivar_id, &context).await
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
    ) -> Result<GetAllCultivarsResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_cultivars(
                color,
                region,
                species,
                prime_name,
                cultivar_name,
                offset,
                limit,
                &context,
            )
            .await
    }

    /// Returns a specific cultivar from the collection by its ID
    async fn get_one_cultivar(&self, cultivar_id: i32) -> Result<GetOneCultivarResponse, ApiError> {
        let context = self.context().clone();
        self.api().get_one_cultivar(cultivar_id, &context).await
    }

    /// Patches a specific cultivar
    async fn patch_one_cultivar(
        &self,
        cultivar_id: i32,
        cultivar_patch: models::CultivarPatch,
    ) -> Result<PatchOneCultivarResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .patch_one_cultivar(cultivar_id, cultivar_patch, &context)
            .await
    }

    /// Adds a new Cultivar to the collection
    async fn post_cultivar(
        &self,
        cultivar: models::Cultivar,
    ) -> Result<PostCultivarResponse, ApiError> {
        let context = self.context().clone();
        self.api().post_cultivar(cultivar, &context).await
    }

    /// Can't delete complete collection
    async fn delete_field(&self) -> Result<DeleteFieldResponse, ApiError> {
        let context = self.context().clone();
        self.api().delete_field(&context).await
    }

    /// Removes a specific stored Field from the collection
    async fn delete_one_field(&self, field_id: i64) -> Result<DeleteOneFieldResponse, ApiError> {
        let context = self.context().clone();
        self.api().delete_one_field(field_id, &context).await
    }

    /// Returns a list of (all) stored Fields
    async fn get_all_fields(
        &self,
        box2d: Option<&Vec<f64>>,
        fuzzy_field_keywords: Option<&Vec<String>>,
        field_name_contains: Option<&Vec<String>>,
        fruit_name_contains: Option<&Vec<String>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllFieldsResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_fields(
                box2d,
                fuzzy_field_keywords,
                field_name_contains,
                fruit_name_contains,
                offset,
                limit,
                &context,
            )
            .await
    }

    /// Returns a specific stored Field from the collection
    async fn get_one_field(&self, field_id: i64) -> Result<GetOneFieldResponse, ApiError> {
        let context = self.context().clone();
        self.api().get_one_field(field_id, &context).await
    }

    /// Patches a specific stored Field
    async fn patch_one_field(
        &self,
        field_id: i64,
        field_patch: models::FieldPatch,
    ) -> Result<PatchOneFieldResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .patch_one_field(field_id, field_patch, &context)
            .await
    }

    /// Adds a new Field to the collection
    async fn post_field(&self, field: models::Field) -> Result<PostFieldResponse, ApiError> {
        let context = self.context().clone();
        self.api().post_field(field, &context).await
    }

    /// Can't delete complete collection
    async fn delete_forecast(&self) -> Result<DeleteForecastResponse, ApiError> {
        let context = self.context().clone();
        self.api().delete_forecast(&context).await
    }

    /// Removes a specific stored Forecast from the collection
    async fn delete_one_forecast(
        &self,
        forecast_id: i64,
    ) -> Result<DeleteOneForecastResponse, ApiError> {
        let context = self.context().clone();
        self.api().delete_one_forecast(forecast_id, &context).await
    }

    /// Returns a list of (all) stored Forecasts
    async fn get_all_forecasts(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllForecastsResponse, ApiError> {
        let context = self.context().clone();
        self.api().get_all_forecasts(offset, limit, &context).await
    }

    /// Returns a specific stored Forecast from the collection
    async fn get_one_forecast(&self, forecast_id: i64) -> Result<GetOneForecastResponse, ApiError> {
        let context = self.context().clone();
        self.api().get_one_forecast(forecast_id, &context).await
    }

    /// Adds a new Forecast to the collection
    async fn post_forecast(
        &self,
        forecast: models::Forecast,
    ) -> Result<PostForecastResponse, ApiError> {
        let context = self.context().clone();
        self.api().post_forecast(forecast, &context).await
    }

    /// Can't delete complete collection
    async fn delete_completed_harvest_task(
        &self,
    ) -> Result<DeleteCompletedHarvestTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api().delete_completed_harvest_task(&context).await
    }

    /// Removes either a specific stored Harvest Task or a specific stored Harvest with Harvest Measurements from the completed collection
    async fn delete_one_completed_harvest_task(
        &self,
        harvest_task_id: i64,
    ) -> Result<DeleteOneCompletedHarvestTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .delete_one_completed_harvest_task(harvest_task_id, &context)
            .await
    }

    /// Removes a specific stored Harvest Task from the scheduled collection
    async fn delete_one_scheduled_harvest_task(
        &self,
        harvest_task_id: i64,
    ) -> Result<DeleteOneScheduledHarvestTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .delete_one_scheduled_harvest_task(harvest_task_id, &context)
            .await
    }

    /// Can't delete complete collection
    async fn delete_scheduled_harvest_task(
        &self,
    ) -> Result<DeleteScheduledHarvestTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api().delete_scheduled_harvest_task(&context).await
    }

    /// Returns a list of (all) stored completed Harvest Tasks and Harvests with Harvest Measurements
    async fn get_all_completed_harvest_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllCompletedHarvestTasksResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_completed_harvest_tasks(box2d, offset, limit, &context)
            .await
    }

    /// Returns a list of (all) stored scheduled Harvest Tasks
    async fn get_all_scheduled_harvest_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllScheduledHarvestTasksResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_scheduled_harvest_tasks(box2d, offset, limit, &context)
            .await
    }

    /// Returns either a specific stored Harvest Task or a specific stored Harvest with Harvest Measurements from the completed collection
    async fn get_one_completed_harvest_task(
        &self,
        harvest_task_id: i64,
    ) -> Result<GetOneCompletedHarvestTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_one_completed_harvest_task(harvest_task_id, &context)
            .await
    }

    /// Returns a specific stored Harvest Task from the scheduled collection
    async fn get_one_scheduled_harvest_task(
        &self,
        harvest_task_id: i64,
    ) -> Result<GetOneScheduledHarvestTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_one_scheduled_harvest_task(harvest_task_id, &context)
            .await
    }

    /// Adds either a new Harvest Task or a new Harvest with Harvest Measurements to the completed collection
    async fn post_completed_harvest_task(
        &self,
        harvest_or_task: models::HarvestOrTask,
    ) -> Result<PostCompletedHarvestTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .post_completed_harvest_task(harvest_or_task, &context)
            .await
    }

    /// Adds a new Harvest Task to the scheduled collection
    async fn post_scheduled_harvest_task(
        &self,
        harvest_task: models::HarvestTask,
    ) -> Result<PostScheduledHarvestTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .post_scheduled_harvest_task(harvest_task, &context)
            .await
    }

    /// Removes a specific stored Plant from the collection
    async fn delete_one_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
    ) -> Result<DeleteOnePlantResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .delete_one_plant(field_id, planter_run_id, plant_id, &context)
            .await
    }

    /// Can't delete complete collection
    async fn delete_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
    ) -> Result<DeletePlantResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .delete_plant(field_id, planter_run_id, &context)
            .await
    }

    /// Returns a list of (all) stored Plants
    async fn get_all_plants(
        &self,
        field_id: i64,
        planter_run_id: i64,
        offset: Option<i32>,
        limit: Option<i32>,
        box2d: Option<&Vec<f64>>,
    ) -> Result<GetAllPlantsResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_plants(field_id, planter_run_id, offset, limit, box2d, &context)
            .await
    }

    /// Returns a specific stored Plant from the collection
    async fn get_one_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
    ) -> Result<GetOnePlantResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_one_plant(field_id, planter_run_id, plant_id, &context)
            .await
    }

    /// Patches a specific stored Plant
    async fn patch_one_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        plant_patch: models::PlantPatch,
    ) -> Result<PatchOnePlantResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .patch_one_plant(field_id, planter_run_id, plant_id, plant_patch, &context)
            .await
    }

    /// Adds a new Plant to the collection
    async fn post_plant(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant: models::Plant,
    ) -> Result<PostPlantResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .post_plant(field_id, planter_run_id, plant, &context)
            .await
    }

    /// Removes a specific stored Planter Run from the collection
    async fn delete_one_planter_run(
        &self,
        field_id: i64,
        planter_run_id: i64,
    ) -> Result<DeleteOnePlanterRunResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .delete_one_planter_run(field_id, planter_run_id, &context)
            .await
    }

    /// Can't delete complete collection
    async fn delete_planter_run(
        &self,
        field_id: i64,
    ) -> Result<DeletePlanterRunResponse, ApiError> {
        let context = self.context().clone();
        self.api().delete_planter_run(field_id, &context).await
    }

    /// Returns a list of (all) stored Planter Runs
    async fn get_all_planter_runs(
        &self,
        field_id: i64,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllPlanterRunsResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_planter_runs(field_id, box2d, offset, limit, &context)
            .await
    }

    /// Returns a specific stored Planter Run from the collection
    async fn get_one_planter_run(
        &self,
        field_id: i64,
        planter_run_id: i64,
    ) -> Result<GetOnePlanterRunResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_one_planter_run(field_id, planter_run_id, &context)
            .await
    }

    /// Adds a new Planter Run to the collection
    async fn post_planter_run(
        &self,
        field_id: i64,
        planter_run: models::PlanterRun,
    ) -> Result<PostPlanterRunResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .post_planter_run(field_id, planter_run, &context)
            .await
    }

    /// Removes a specific stored Quality Measurement from the collection
    async fn delete_one_quality_measurement(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        quality_measurement_id: i64,
    ) -> Result<DeleteOneQualityMeasurementResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .delete_one_quality_measurement(
                field_id,
                planter_run_id,
                plant_id,
                quality_measurement_id,
                &context,
            )
            .await
    }

    /// Can't delete complete collection
    async fn delete_quality_measurement(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
    ) -> Result<DeleteQualityMeasurementResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .delete_quality_measurement(field_id, planter_run_id, plant_id, &context)
            .await
    }

    /// Returns a list of (all) stored Quality Measurements
    async fn get_all_quality_measurements(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllQualityMeasurementsResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_quality_measurements(
                field_id,
                planter_run_id,
                plant_id,
                offset,
                limit,
                &context,
            )
            .await
    }

    /// Returns a specific stored Quality Measurement from the collection
    async fn get_one_quality_measurement(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        quality_measurement_id: i64,
    ) -> Result<GetOneQualityMeasurementResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_one_quality_measurement(
                field_id,
                planter_run_id,
                plant_id,
                quality_measurement_id,
                &context,
            )
            .await
    }

    /// Adds a new Quality Measurement to the collection
    async fn post_quality_measurement(
        &self,
        field_id: i64,
        planter_run_id: i64,
        plant_id: i64,
        quality_measurement: models::QualityMeasurement,
    ) -> Result<PostQualityMeasurementResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .post_quality_measurement(
                field_id,
                planter_run_id,
                plant_id,
                quality_measurement,
                &context,
            )
            .await
    }

    /// Can't delete complete collection
    async fn delete_completed_spray_task(
        &self,
    ) -> Result<DeleteCompletedSprayTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api().delete_completed_spray_task(&context).await
    }

    /// Removes either a specific stored Spray Task or a specific stored Spray with Spray Measurements from the completed collection
    async fn delete_one_completed_spray_task(
        &self,
        spray_task_id: i64,
    ) -> Result<DeleteOneCompletedSprayTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .delete_one_completed_spray_task(spray_task_id, &context)
            .await
    }

    /// Removes a specific stored Spray Task from the scheduled collection
    async fn delete_one_scheduled_spray_task(
        &self,
        spray_task_id: i64,
    ) -> Result<DeleteOneScheduledSprayTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .delete_one_scheduled_spray_task(spray_task_id, &context)
            .await
    }

    /// Can't delete complete collection
    async fn delete_scheduled_spray_task(
        &self,
    ) -> Result<DeleteScheduledSprayTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api().delete_scheduled_spray_task(&context).await
    }

    /// Returns a list of (all) stored completed Spray Tasks and Sprays with Spray Measurements
    async fn get_all_completed_spray_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllCompletedSprayTasksResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_completed_spray_tasks(box2d, offset, limit, &context)
            .await
    }

    /// Returns a list of (all) stored scheduled Spray Tasks
    async fn get_all_scheduled_spray_tasks(
        &self,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllScheduledSprayTasksResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_scheduled_spray_tasks(box2d, offset, limit, &context)
            .await
    }

    /// Returns either a specific stored Spray Task or a specific stored Spray with Spray Measurements from the completed collection
    async fn get_one_completed_spray_task(
        &self,
        spray_task_id: i64,
    ) -> Result<GetOneCompletedSprayTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_one_completed_spray_task(spray_task_id, &context)
            .await
    }

    /// Returns a specific stored Spray Task from the scheduled collection
    async fn get_one_scheduled_spray_task(
        &self,
        spray_task_id: i64,
    ) -> Result<GetOneScheduledSprayTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_one_scheduled_spray_task(spray_task_id, &context)
            .await
    }

    /// Adds either a new Spray Task or a new Spray with Spray Measurements to the completed collection
    async fn post_completed_spray_task(
        &self,
        spray_task: models::SprayTask,
    ) -> Result<PostCompletedSprayTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .post_completed_spray_task(spray_task, &context)
            .await
    }

    /// Adds a new Spray Task to the scheduled collection
    async fn post_scheduled_spray_task(
        &self,
        spray_task: models::SprayTask,
    ) -> Result<PostScheduledSprayTaskResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .post_scheduled_spray_task(spray_task, &context)
            .await
    }

    /// Removes a specific stored Terrain Model from the collection
    async fn delete_one_terrain(
        &self,
        terrain_id: i64,
    ) -> Result<DeleteOneTerrainResponse, ApiError> {
        let context = self.context().clone();
        self.api().delete_one_terrain(terrain_id, &context).await
    }

    /// Can't delete complete collection
    async fn delete_terrain(&self) -> Result<DeleteTerrainResponse, ApiError> {
        let context = self.context().clone();
        self.api().delete_terrain(&context).await
    }

    /// Returns a list of (all) stored Terrain Models
    async fn get_all_terrains(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllTerrainsResponse, ApiError> {
        let context = self.context().clone();
        self.api().get_all_terrains(offset, limit, &context).await
    }

    /// Returns a specific stored Terrain Model from the collection
    async fn get_one_terrain(&self, terrain_id: i64) -> Result<GetOneTerrainResponse, ApiError> {
        let context = self.context().clone();
        self.api().get_one_terrain(terrain_id, &context).await
    }

    /// Deletes a specific access right for a specific token
    async fn delete_one_access_right(
        &self,
        user_id: i32,
        tokenstr: String,
        access_right_path: String,
    ) -> Result<DeleteOneAccessRightResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .delete_one_access_right(user_id, tokenstr, access_right_path, &context)
            .await
    }

    /// Deletes a specific token
    async fn delete_one_token(
        &self,
        user_id: i32,
        tokenstr: String,
    ) -> Result<DeleteOneTokenResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .delete_one_token(user_id, tokenstr, &context)
            .await
    }

    /// Deletes the user and all its tokens
    async fn delete_one_user(&self, user_id: i32) -> Result<DeleteOneUserResponse, ApiError> {
        let context = self.context().clone();
        self.api().delete_one_user(user_id, &context).await
    }

    /// Returns a list of access rights for a specific token
    async fn get_access_rights(
        &self,
        user_id: i32,
        tokenstr: String,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAccessRightsResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_access_rights(user_id, tokenstr, offset, limit, &context)
            .await
    }

    /// Returns a list of all existing tokens
    async fn get_all_tokens(
        &self,
        tokenstr_contains: Option<&Vec<String>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllTokensResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_tokens(tokenstr_contains, offset, limit, &context)
            .await
    }

    /// Returns information about a specific access right for a specific token
    async fn get_one_access_right(
        &self,
        user_id: i32,
        tokenstr: String,
        access_right_path: String,
    ) -> Result<GetOneAccessRightResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_one_access_right(user_id, tokenstr, access_right_path, &context)
            .await
    }

    /// Returns all tokens of the user
    async fn get_user_tokens(
        &self,
        user_id: i32,
        tokenstr_contains: Option<&Vec<String>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetUserTokensResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_user_tokens(user_id, tokenstr_contains, offset, limit, &context)
            .await
    }

    /// Creates a new access right for a specific token
    async fn post_access_right(
        &self,
        user_id: i32,
        tokenstr: String,
        access_right: models::AccessRight,
    ) -> Result<PostAccessRightResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .post_access_right(user_id, tokenstr, access_right, &context)
            .await
    }

    /// Creates a new token
    async fn post_token(&self, token: models::Token) -> Result<PostTokenResponse, ApiError> {
        let context = self.context().clone();
        self.api().post_token(token, &context).await
    }

    /// Creates a new token of the given `tokenstr`
    async fn post_user_token(
        &self,
        user_id: i32,
        user_token: models::UserToken,
    ) -> Result<PostUserTokenResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .post_user_token(user_id, user_token, &context)
            .await
    }

    /// Removes weather data for a specific station and date
    async fn delete_one_weather_data(
        &self,
        weather_station_id: i32,
        measurement_date: String,
    ) -> Result<DeleteOneWeatherDataResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .delete_one_weather_data(weather_station_id, measurement_date, &context)
            .await
    }

    /// Returns a list of weather data
    async fn get_all_weather_data(
        &self,
        station_ids: Option<&Vec<i32>>,
        start_date: Option<String>,
        end_date: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllWeatherDataResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_weather_data(station_ids, start_date, end_date, offset, limit, &context)
            .await
    }

    /// Returns weather data for a specific station and date
    async fn get_one_weather_data(
        &self,
        weather_station_id: i32,
        measurement_date: String,
    ) -> Result<GetOneWeatherDataResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_one_weather_data(weather_station_id, measurement_date, &context)
            .await
    }

    /// Patches weather data for a specific station and date
    async fn patch_one_weather_data(
        &self,
        weather_station_id: i32,
        measurement_date: String,
        weather_data_patch: models::WeatherDataPatch,
    ) -> Result<PatchOneWeatherDataResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .patch_one_weather_data(
                weather_station_id,
                measurement_date,
                weather_data_patch,
                &context,
            )
            .await
    }

    /// Adds new weather data
    async fn post_weather_data(
        &self,
        weather_data: models::WeatherData,
    ) -> Result<PostWeatherDataResponse, ApiError> {
        let context = self.context().clone();
        self.api().post_weather_data(weather_data, &context).await
    }

    /// Removes a specific weather station from the collection
    async fn delete_one_weather_station(
        &self,
        weather_station_id: i32,
    ) -> Result<DeleteOneWeatherStationResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .delete_one_weather_station(weather_station_id, &context)
            .await
    }

    /// Returns a list of all weather stations
    async fn get_all_weather_stations(
        &self,
        bundesland: Option<&Vec<String>>,
        name: Option<&Vec<String>>,
        box2d: Option<&Vec<f64>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetAllWeatherStationsResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_all_weather_stations(bundesland, name, box2d, offset, limit, &context)
            .await
    }

    /// Returns a specific weather station from the collection
    async fn get_one_weather_station(
        &self,
        weather_station_id: i32,
    ) -> Result<GetOneWeatherStationResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_one_weather_station(weather_station_id, &context)
            .await
    }

    /// Patches a specific weather station
    async fn patch_one_weather_station(
        &self,
        weather_station_id: i32,
        weather_station_patch: models::WeatherStationPatch,
    ) -> Result<PatchOneWeatherStationResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .patch_one_weather_station(weather_station_id, weather_station_patch, &context)
            .await
    }

    /// Adds a new Weather Station to the collection
    async fn post_weather_station(
        &self,
        weather_station: models::WeatherStation,
    ) -> Result<PostWeatherStationResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .post_weather_station(weather_station, &context)
            .await
    }
}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
