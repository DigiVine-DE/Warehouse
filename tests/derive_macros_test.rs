use digivine_server::models::{self, Error as ModelsError};
use macros_lib::Response;
use std::{error::Error, fmt::Display};

#[test]
fn basic_resp_test() {
    #[allow(dead_code)]
    #[derive(Debug, PartialEq, Response)]
    enum BasicResp {
        SuccessfulOperation(InlineResponse),
        BadRequest(models::InlineResponse400),
        MissingAPIKey,
        SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
        BackendError(ModelsError),
    }

    let backend_ok: Option<i64> = Some(1);
    let resp_ok = BasicResp::SuccessfulOperation(InlineResponse { id: backend_ok });

    let backend_err = BackendError("Some Error");
    let resp_err = BasicResp::BackendError(ModelsError::new(
        "BasicResp".to_string(),
        "Some Error".to_string(),
    ));

    assert_eq!(BasicResp::from(backend_ok), resp_ok);

    let from_backend_err: BasicResp = backend_err.as_backend_error_resp();
    assert_eq!(from_backend_err, resp_err);

    let res: Result<Option<i64>, BackendError> = Ok(Some(1));
    assert_eq!(BasicResp::from(res), resp_ok);

    let res: Result<Option<i64>, BackendError> = Err(BackendError("Some Error"));
    assert_eq!(BasicResp::from(res), resp_err)
}

#[test]
fn vec_resp_test() {
    #[allow(dead_code)]
    #[derive(Debug, PartialEq, Response)]
    enum VecResp {
        SuccessfulOperation(Vec<InlineResponse>),
        BadRequest(models::InlineResponse400),
        MissingAPIKey,
        SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
        BackendError(ModelsError),
    }

    let backend_ok: Vec<Option<i64>> = vec![Some(1)];
    let resp_ok = VecResp::SuccessfulOperation(vec![InlineResponse { id: Some(1) }]);

    let backend_err = BackendError("Some Error");
    let resp_err = VecResp::BackendError(ModelsError::new(
        "VecResp".to_string(),
        "Some Error".to_string(),
    ));

    let from_backend_ok: VecResp = backend_ok.into_iter().collect();
    assert_eq!(from_backend_ok, resp_ok);

    let from_backend_err: VecResp = backend_err.as_backend_error_resp();
    assert_eq!(from_backend_err, resp_err);
}

#[test]
fn unit_resp_test() {
    #[allow(dead_code)]
    #[derive(Debug, PartialEq, Response)]
    enum UnitResp {
        SuccessfulOperation,
        BadRequest(models::InlineResponse400),
        MissingAPIKey,
        SpecifiedAPIKeyDoesNotHaveTheRequiredRights,
        BackendError(ModelsError),
    }

    let res: Result<(), BackendError> = Ok(());
    assert_eq!(UnitResp::from(res), UnitResp::SuccessfulOperation);

    let backend_err = BackendError("Some Error");
    let resp_err = UnitResp::BackendError(ModelsError::new(
        "UnitResp".to_string(),
        "Some Error".to_string(),
    ));

    let from_backend_err: UnitResp = backend_err.as_backend_error_resp();
    assert_eq!(from_backend_err, resp_err);

    let res: Result<(), BackendError> = Err(backend_err);
    assert_eq!(UnitResp::from(res), resp_err)
}

// mimic the error type in backend
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BackendError(pub &'static str);

impl Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl Error for BackendError {}

#[derive(Clone, Debug)]
pub struct ApiError(pub String);

// mimic inline response in generated server
#[derive(Debug, Clone, PartialEq, Default)]
pub struct InlineResponse {
    pub id: Option<i64>,
}

impl From<Option<i64>> for InlineResponse {
    fn from(value: Option<i64>) -> Self {
        Self { id: value }
    }
}

pub trait AsBackendErrorResp<T> {
    fn default_code() -> &'static str;
    fn as_backend_error_resp_with_code(&self, code: String) -> T;

    fn as_backend_error_resp(&self) -> T {
        self.as_backend_error_resp_with_code(Self::default_code().to_string())
    }
}

pub trait NotFoundResponse {
    fn not_found() -> Self;
}

pub trait UnitSuccessResponse {
    fn succeed() -> Self;
}

pub trait AsBadRequestResponse<T> {
    fn as_bad_request(self) -> T;
}
