use async_trait::async_trait;
use futures::{
    future,
    future::{BoxFuture, FutureExt, TryFutureExt},
    stream,
    stream::StreamExt,
    Stream,
};
use hyper::{
    header::{HeaderName, HeaderValue, CONTENT_TYPE},
    service::Service,
    Body, Request, Response, Uri,
};
use percent_encoding::{utf8_percent_encode, AsciiSet};
use std::{
    borrow::Cow,
    convert::TryInto,
    error::Error,
    fmt,
    future::Future,
    io::{ErrorKind, Read},
    marker::PhantomData,
    path::Path,
    str,
    str::FromStr,
    string::ToString,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};
use swagger::{ApiError, AuthData, BodyExt, Connector, DropContextService, Has, XSpanIdString};
use url::form_urlencoded;

use crate::{header, models};

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
#[allow(dead_code)]
const FRAGMENT_ENCODE_SET: &AsciiSet = &percent_encoding::CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`');

/// This encode set is used for object IDs
///
/// Aside from the special characters defined in the `PATH_SEGMENT_ENCODE_SET`,
/// the vertical bar (|) is encoded.
#[allow(dead_code)]
const ID_ENCODE_SET: &AsciiSet = &FRAGMENT_ENCODE_SET.add(b'|');

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

/// Convert input into a base path, e.g. "http://example:123". Also checks the scheme as it goes.
fn into_base_path(
    input: impl TryInto<Uri, Error = hyper::http::uri::InvalidUri>,
    correct_scheme: Option<&'static str>,
) -> Result<String, ClientInitError> {
    // First convert to Uri, since a base path is a subset of Uri.
    let uri = input.try_into()?;

    let scheme = uri.scheme_str().ok_or(ClientInitError::InvalidScheme)?;

    // Check the scheme if necessary
    if let Some(correct_scheme) = correct_scheme {
        if scheme != correct_scheme {
            return Err(ClientInitError::InvalidScheme);
        }
    }

    let host = uri.host().ok_or_else(|| ClientInitError::MissingHost)?;
    let port = uri
        .port_u16()
        .map(|x| format!(":{}", x))
        .unwrap_or_default();
    Ok(format!(
        "{}://{}{}{}",
        scheme,
        host,
        port,
        uri.path().trim_end_matches('/')
    ))
}

/// A client that implements the API by making HTTP calls out to a server.
pub struct Client<S, C>
where
    S: Service<(Request<Body>, C), Response = Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static,
{
    /// Inner service
    client_service: S,

    /// Base path of the API
    base_path: String,

    /// Marker
    marker: PhantomData<fn(C)>,
}

impl<S, C> fmt::Debug for Client<S, C>
where
    S: Service<(Request<Body>, C), Response = Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Client {{ base_path: {} }}", self.base_path)
    }
}

impl<S, C> Clone for Client<S, C>
where
    S: Service<(Request<Body>, C), Response = Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            client_service: self.client_service.clone(),
            base_path: self.base_path.clone(),
            marker: PhantomData,
        }
    }
}

impl<Connector, C> Client<DropContextService<hyper::client::Client<Connector, Body>, C>, C>
where
    Connector: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Create a client with a custom implementation of hyper::client::Connect.
    ///
    /// Intended for use with custom implementations of connect for e.g. protocol logging
    /// or similar functionality which requires wrapping the transport layer. When wrapping a TCP connection,
    /// this function should be used in conjunction with `swagger::Connector::builder()`.
    ///
    /// For ordinary tcp connections, prefer the use of `try_new_http`, `try_new_https`
    /// and `try_new_https_mutual`, to avoid introducing a dependency on the underlying transport layer.
    ///
    /// # Arguments
    ///
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    /// * `protocol` - Which protocol to use when constructing the request url, e.g. `Some("http")`
    /// * `connector` - Implementation of `hyper::client::Connect` to use for the client
    pub fn try_new_with_connector(
        base_path: &str,
        protocol: Option<&'static str>,
        connector: Connector,
    ) -> Result<Self, ClientInitError> {
        let client_service = hyper::client::Client::builder().build(connector);
        let client_service = DropContextService::new(client_service);

        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, protocol)?,
            marker: PhantomData,
        })
    }
}

#[derive(Debug, Clone)]
pub enum HyperClient {
    Http(hyper::client::Client<hyper::client::HttpConnector, Body>),
    Https(hyper::client::Client<HttpsConnector, Body>),
}

impl Service<Request<Body>> for HyperClient {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = hyper::client::ResponseFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        match self {
            HyperClient::Http(client) => client.poll_ready(cx),
            HyperClient::Https(client) => client.poll_ready(cx),
        }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        match self {
            HyperClient::Http(client) => client.call(req),
            HyperClient::Https(client) => client.call(req),
        }
    }
}

impl<C> Client<DropContextService<HyperClient, C>, C>
where
    C: Clone + Send + Sync + 'static,
{
    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    pub fn try_new(base_path: &str) -> Result<Self, ClientInitError> {
        let uri = Uri::from_str(base_path)?;

        let scheme = uri.scheme_str().ok_or(ClientInitError::InvalidScheme)?;
        let scheme = scheme.to_ascii_lowercase();

        let connector = Connector::builder();

        let client_service = match scheme.as_str() {
            "http" => HyperClient::Http(hyper::client::Client::builder().build(connector.build())),
            "https" => {
                let connector = connector
                    .https()
                    .build()
                    .map_err(|e| ClientInitError::SslError(e))?;
                HyperClient::Https(hyper::client::Client::builder().build(connector))
            }
            _ => {
                return Err(ClientInitError::InvalidScheme);
            }
        };

        let client_service = DropContextService::new(client_service);

        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, None)?,
            marker: PhantomData,
        })
    }
}

impl<C> Client<DropContextService<hyper::client::Client<hyper::client::HttpConnector, Body>, C>, C>
where
    C: Clone + Send + Sync + 'static,
{
    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    pub fn try_new_http(base_path: &str) -> Result<Self, ClientInitError> {
        let http_connector = Connector::builder().build();

        Self::try_new_with_connector(base_path, Some("http"), http_connector)
    }
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
type HttpsConnector = hyper_openssl::HttpsConnector<hyper::client::HttpConnector>;

impl<C> Client<DropContextService<hyper::client::Client<HttpsConnector, Body>, C>, C>
where
    C: Clone + Send + Sync + 'static,
{
    /// Create a client with a TLS connection to the server
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    pub fn try_new_https(base_path: &str) -> Result<Self, ClientInitError> {
        let https_connector = Connector::builder()
            .https()
            .build()
            .map_err(|e| ClientInitError::SslError(e))?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }

    /// Create a client with a TLS connection to the server using a pinned certificate
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    pub fn try_new_https_pinned<CA>(
        base_path: &str,
        ca_certificate: CA,
    ) -> Result<Self, ClientInitError>
    where
        CA: AsRef<Path>,
    {
        let https_connector = Connector::builder()
            .https()
            .pin_server_certificate(ca_certificate)
            .build()
            .map_err(|e| ClientInitError::SslError(e))?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }

    /// Create a client with a mutually authenticated TLS connection to the server.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    /// * `client_key` - Path to the client private key
    /// * `client_certificate` - Path to the client's public certificate associated with the private key
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    pub fn try_new_https_mutual<CA, K, D>(
        base_path: &str,
        ca_certificate: CA,
        client_key: K,
        client_certificate: D,
    ) -> Result<Self, ClientInitError>
    where
        CA: AsRef<Path>,
        K: AsRef<Path>,
        D: AsRef<Path>,
    {
        let https_connector = Connector::builder()
            .https()
            .pin_server_certificate(ca_certificate)
            .client_authentication(client_key, client_certificate)
            .build()
            .map_err(|e| ClientInitError::SslError(e))?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }
}

impl<S, C> Client<S, C>
where
    S: Service<(Request<Body>, C), Response = Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static,
{
    /// Constructor for creating a `Client` by passing in a pre-made `hyper::service::Service` /
    /// `tower::Service`
    ///
    /// This allows adding custom wrappers around the underlying transport, for example for logging.
    pub fn try_new_with_client_service(
        client_service: S,
        base_path: &str,
    ) -> Result<Self, ClientInitError> {
        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, None)?,
            marker: PhantomData,
        })
    }
}

/// Error type failing to create a Client
#[derive(Debug)]
pub enum ClientInitError {
    /// Invalid URL Scheme
    InvalidScheme,

    /// Invalid URI
    InvalidUri(hyper::http::uri::InvalidUri),

    /// Missing Hostname
    MissingHost,

    /// SSL Connection Error
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
    SslError(native_tls::Error),

    /// SSL Connection Error
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    SslError(openssl::error::ErrorStack),
}

impl From<hyper::http::uri::InvalidUri> for ClientInitError {
    fn from(err: hyper::http::uri::InvalidUri) -> ClientInitError {
        ClientInitError::InvalidUri(err)
    }
}

impl fmt::Display for ClientInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &dyn fmt::Debug = self;
        s.fmt(f)
    }
}

impl Error for ClientInitError {
    fn description(&self) -> &str {
        "Failed to produce a hyper client."
    }
}

#[async_trait]
impl<S, C> Api<C> for Client<S, C>
where
    S: Service<(Request<Body>, C), Response = Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Has<XSpanIdString> + Has<Option<AuthData>> + Clone + Send + Sync + 'static,
{
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), crate::ServiceError>> {
        match self.client_service.clone().poll_ready(cx) {
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok(o)) => Poll::Ready(Ok(o)),
            Poll::Pending => Poll::Pending,
        }
    }

    async fn delete_one_cultivar(
        &self,
        param_cultivar_id: i32,
        context: &C,
    ) -> Result<DeleteOneCultivarResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/vivc/{cultivar_id}",
            self.base_path,
            cultivar_id = utf8_percent_encode(&param_cultivar_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOneCultivarResponse::SuccessfullyDeletedCultivar)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOneCultivarResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOneCultivarResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteOneCultivarResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOneCultivarResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOneCultivarResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_cultivars(
        &self,
        param_color: Option<&Vec<String>>,
        param_region: Option<&Vec<String>>,
        param_species: Option<&Vec<String>>,
        param_prime_name: Option<&Vec<String>>,
        param_cultivar_name: Option<&Vec<String>>,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllCultivarsResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/vivc", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_color) = param_color {
                query_string.append_pair(
                    "color",
                    &param_color
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_region) = param_region {
                query_string.append_pair(
                    "region",
                    &param_region
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_species) = param_species {
                query_string.append_pair(
                    "species",
                    &param_species
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_prime_name) = param_prime_name {
                query_string.append_pair(
                    "primeName",
                    &param_prime_name
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_cultivar_name) = param_cultivar_name {
                query_string.append_pair(
                    "cultivarName",
                    &param_cultivar_name
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::Cultivar>>(body)?;
                Ok(GetAllCultivarsResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAllCultivarsResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAllCultivarsResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetAllCultivarsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAllCultivarsResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_one_cultivar(
        &self,
        param_cultivar_id: i32,
        context: &C,
    ) -> Result<GetOneCultivarResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/vivc/{cultivar_id}",
            self.base_path,
            cultivar_id = utf8_percent_encode(&param_cultivar_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Cultivar>(body)?;
                Ok(GetOneCultivarResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetOneCultivarResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetOneCultivarResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetOneCultivarResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetOneCultivarResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetOneCultivarResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn patch_one_cultivar(
        &self,
        param_cultivar_id: i32,
        param_cultivar_patch: models::CultivarPatch,
        context: &C,
    ) -> Result<PatchOneCultivarResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/vivc/{cultivar_id}",
            self.base_path,
            cultivar_id = utf8_percent_encode(&param_cultivar_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("PATCH")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_cultivar_patch).expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Cultivar>(body)?;
                Ok(PatchOneCultivarResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PatchOneCultivarResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PatchOneCultivarResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PatchOneCultivarResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(PatchOneCultivarResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PatchOneCultivarResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_cultivar(
        &self,
        param_cultivar: models::Cultivar,
        context: &C,
    ) -> Result<PostCultivarResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/vivc", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_cultivar).expect("impossible to fail to serialize");

        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse2017>(body)?;
                Ok(PostCultivarResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostCultivarResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostCultivarResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostCultivarResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostCultivarResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_field(&self, context: &C) -> Result<DeleteFieldResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/field", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteFieldResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteFieldResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteFieldResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            405 => {
                let body = response.into_body();
                Ok(DeleteFieldResponse::MethodNotAllowed)
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_field(
        &self,
        param_field_id: i64,
        context: &C,
    ) -> Result<DeleteOneFieldResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}",
            self.base_path,
            field_id = utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOneFieldResponse::SuccessfullyDeletedField)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOneFieldResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOneFieldResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteOneFieldResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOneFieldResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOneFieldResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_fields(
        &self,
        param_box2d: Option<&Vec<f64>>,
        param_fuzzy_field_keywords: Option<&Vec<String>>,
        param_field_name_contains: Option<&Vec<String>>,
        param_fruit_name_contains: Option<&Vec<String>>,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllFieldsResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/field", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_box2d) = param_box2d {
                query_string.append_pair(
                    "box2d",
                    &param_box2d
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_fuzzy_field_keywords) = param_fuzzy_field_keywords {
                query_string.append_pair(
                    "fuzzyFieldKeywords",
                    &param_fuzzy_field_keywords
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_field_name_contains) = param_field_name_contains {
                query_string.append_pair(
                    "fieldNameContains",
                    &param_field_name_contains
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_fruit_name_contains) = param_fruit_name_contains {
                query_string.append_pair(
                    "fruitNameContains",
                    &param_fruit_name_contains
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::Field>>(body)?;
                Ok(GetAllFieldsResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAllFieldsResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAllFieldsResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetAllFieldsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAllFieldsResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_one_field(
        &self,
        param_field_id: i64,
        context: &C,
    ) -> Result<GetOneFieldResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}",
            self.base_path,
            field_id = utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Field>(body)?;
                Ok(GetOneFieldResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetOneFieldResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetOneFieldResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetOneFieldResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetOneFieldResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetOneFieldResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn patch_one_field(
        &self,
        param_field_id: i64,
        param_field_patch: models::FieldPatch,
        context: &C,
    ) -> Result<PatchOneFieldResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}",
            self.base_path,
            field_id = utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("PATCH")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_field_patch).expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Field>(body)?;
                Ok(PatchOneFieldResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PatchOneFieldResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PatchOneFieldResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PatchOneFieldResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(PatchOneFieldResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PatchOneFieldResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_field(
        &self,
        param_field: models::Field,
        context: &C,
    ) -> Result<PostFieldResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/field", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_field).expect("impossible to fail to serialize");

        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse2011>(body)?;
                Ok(PostFieldResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostFieldResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostFieldResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostFieldResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostFieldResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_forecast(&self, context: &C) -> Result<DeleteForecastResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/forecast", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteForecastResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteForecastResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteForecastResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            405 => {
                let body = response.into_body();
                Ok(DeleteForecastResponse::MethodNotAllowed)
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_forecast(
        &self,
        param_forecast_id: i64,
        context: &C,
    ) -> Result<DeleteOneForecastResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/forecast/{forecast_id}",
            self.base_path,
            forecast_id = utf8_percent_encode(&param_forecast_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOneForecastResponse::SuccessfullyDeletedForecast)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOneForecastResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOneForecastResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteOneForecastResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOneForecastResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOneForecastResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_forecasts(
        &self,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllForecastsResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/forecast", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::Forecast>>(body)?;
                Ok(GetAllForecastsResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAllForecastsResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAllForecastsResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetAllForecastsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAllForecastsResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_one_forecast(
        &self,
        param_forecast_id: i64,
        context: &C,
    ) -> Result<GetOneForecastResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/forecast/{forecast_id}",
            self.base_path,
            forecast_id = utf8_percent_encode(&param_forecast_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Forecast>(body)?;
                Ok(GetOneForecastResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetOneForecastResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetOneForecastResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetOneForecastResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetOneForecastResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetOneForecastResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_forecast(
        &self,
        param_forecast: models::Forecast,
        context: &C,
    ) -> Result<PostForecastResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/forecast", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_forecast).expect("impossible to fail to serialize");

        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse201>(body)?;
                Ok(PostForecastResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostForecastResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostForecastResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostForecastResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostForecastResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_completed_harvest_task(
        &self,
        context: &C,
    ) -> Result<DeleteCompletedHarvestTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/harvest/complete", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteCompletedHarvestTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteCompletedHarvestTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteCompletedHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            405 => {
                let body = response.into_body();
                Ok(DeleteCompletedHarvestTaskResponse::MethodNotAllowed)
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_completed_harvest_task(
        &self,
        param_harvest_task_id: i64,
        context: &C,
    ) -> Result<DeleteOneCompletedHarvestTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/harvest/complete/{harvest_task_id}",
            self.base_path,
            harvest_task_id =
                utf8_percent_encode(&param_harvest_task_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOneCompletedHarvestTaskResponse::SuccessfullyDeletedCompletedHarvest)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOneCompletedHarvestTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOneCompletedHarvestTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(
                    DeleteOneCompletedHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                )
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOneCompletedHarvestTaskResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOneCompletedHarvestTaskResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_scheduled_harvest_task(
        &self,
        param_harvest_task_id: i64,
        context: &C,
    ) -> Result<DeleteOneScheduledHarvestTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/harvest/task/{harvest_task_id}",
            self.base_path,
            harvest_task_id =
                utf8_percent_encode(&param_harvest_task_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOneScheduledHarvestTaskResponse::SuccessfullyDeletedScheduledHarvestTask)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOneScheduledHarvestTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOneScheduledHarvestTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(
                    DeleteOneScheduledHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                )
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOneScheduledHarvestTaskResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOneScheduledHarvestTaskResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_scheduled_harvest_task(
        &self,
        context: &C,
    ) -> Result<DeleteScheduledHarvestTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/harvest/task", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteScheduledHarvestTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteScheduledHarvestTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteScheduledHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            405 => {
                let body = response.into_body();
                Ok(DeleteScheduledHarvestTaskResponse::MethodNotAllowed)
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_completed_harvest_tasks(
        &self,
        param_box2d: Option<&Vec<f64>>,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllCompletedHarvestTasksResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/harvest/complete", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_box2d) = param_box2d {
                query_string.append_pair(
                    "box2d",
                    &param_box2d
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::HarvestOrTask>>(body)?;
                Ok(GetAllCompletedHarvestTasksResponse::SuccessfulOperation(
                    body,
                ))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAllCompletedHarvestTasksResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAllCompletedHarvestTasksResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(
                    GetAllCompletedHarvestTasksResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                )
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAllCompletedHarvestTasksResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_scheduled_harvest_tasks(
        &self,
        param_box2d: Option<&Vec<f64>>,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllScheduledHarvestTasksResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/harvest/task", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_box2d) = param_box2d {
                query_string.append_pair(
                    "box2d",
                    &param_box2d
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::HarvestTask>>(body)?;
                Ok(GetAllScheduledHarvestTasksResponse::SuccessfulOperation(
                    body,
                ))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAllScheduledHarvestTasksResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAllScheduledHarvestTasksResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(
                    GetAllScheduledHarvestTasksResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                )
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAllScheduledHarvestTasksResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_one_completed_harvest_task(
        &self,
        param_harvest_task_id: i64,
        context: &C,
    ) -> Result<GetOneCompletedHarvestTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/harvest/complete/{harvest_task_id}",
            self.base_path,
            harvest_task_id =
                utf8_percent_encode(&param_harvest_task_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HarvestOrTask>(body)?;
                Ok(GetOneCompletedHarvestTaskResponse::SuccessfulOperation(
                    body,
                ))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetOneCompletedHarvestTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetOneCompletedHarvestTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetOneCompletedHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetOneCompletedHarvestTaskResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetOneCompletedHarvestTaskResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_one_scheduled_harvest_task(
        &self,
        param_harvest_task_id: i64,
        context: &C,
    ) -> Result<GetOneScheduledHarvestTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/harvest/task/{harvest_task_id}",
            self.base_path,
            harvest_task_id =
                utf8_percent_encode(&param_harvest_task_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::HarvestTask>(body)?;
                Ok(GetOneScheduledHarvestTaskResponse::SuccessfulOperation(
                    body,
                ))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetOneScheduledHarvestTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetOneScheduledHarvestTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetOneScheduledHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetOneScheduledHarvestTaskResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetOneScheduledHarvestTaskResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_completed_harvest_task(
        &self,
        param_harvest_or_task: models::HarvestOrTask,
        context: &C,
    ) -> Result<PostCompletedHarvestTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/harvest/complete", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_harvest_or_task).expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse2015>(body)?;
                Ok(PostCompletedHarvestTaskResponse::SuccessfulOperation(body))
            }
            303 => {
                let response_location = match response
                    .headers()
                    .get(HeaderName::from_static("location"))
                {
                    Some(response_location) => {
                        let response_location = response_location.clone();
                        let response_location =
                            match TryInto::<header::IntoHeaderValue<String>>::try_into(
                                response_location,
                            ) {
                                Ok(value) => value,
                                Err(e) => {
                                    return Err(ApiError(format!(
                                        "Invalid response header Location for response 303 - {}",
                                        e
                                    )));
                                }
                            };
                        let response_location = response_location.0;
                        Some(response_location)
                    }
                    None => None,
                };

                let body = response.into_body();
                Ok(PostCompletedHarvestTaskResponse::SeeOther {
                    location: response_location,
                })
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostCompletedHarvestTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostCompletedHarvestTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostCompletedHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostCompletedHarvestTaskResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_scheduled_harvest_task(
        &self,
        param_harvest_task: models::HarvestTask,
        context: &C,
    ) -> Result<PostScheduledHarvestTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/harvest/task", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_harvest_task).expect("impossible to fail to serialize");

        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse2015>(body)?;
                Ok(PostScheduledHarvestTaskResponse::SuccessfulOperation(body))
            }
            303 => {
                let response_location = match response
                    .headers()
                    .get(HeaderName::from_static("location"))
                {
                    Some(response_location) => {
                        let response_location = response_location.clone();
                        let response_location =
                            match TryInto::<header::IntoHeaderValue<String>>::try_into(
                                response_location,
                            ) {
                                Ok(value) => value,
                                Err(e) => {
                                    return Err(ApiError(format!(
                                        "Invalid response header Location for response 303 - {}",
                                        e
                                    )));
                                }
                            };
                        let response_location = response_location.0;
                        Some(response_location)
                    }
                    None => None,
                };

                let body = response.into_body();
                Ok(PostScheduledHarvestTaskResponse::SeeOther {
                    location: response_location,
                })
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostScheduledHarvestTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostScheduledHarvestTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostScheduledHarvestTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostScheduledHarvestTaskResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_plant(
        &self,
        param_field_id: i64,
        param_planter_run_id: i64,
        param_plant_id: i64,
        context: &C,
    ) -> Result<DeleteOnePlantResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter/{planter_run_id}/plant/{plant_id}",
            self.base_path,
            field_id = utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET),
            planter_run_id = utf8_percent_encode(&param_planter_run_id.to_string(), ID_ENCODE_SET),
            plant_id = utf8_percent_encode(&param_plant_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOnePlantResponse::SuccessfullyDeletedPlant)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOnePlantResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOnePlantResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteOnePlantResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOnePlantResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOnePlantResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_plant(
        &self,
        param_field_id: i64,
        param_planter_run_id: i64,
        context: &C,
    ) -> Result<DeletePlantResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter/{planter_run_id}/plant",
            self.base_path,
            field_id = utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET),
            planter_run_id = utf8_percent_encode(&param_planter_run_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeletePlantResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeletePlantResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeletePlantResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            405 => {
                let body = response.into_body();
                Ok(DeletePlantResponse::MethodNotAllowed)
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_plants(
        &self,
        param_field_id: i64,
        param_planter_run_id: i64,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        param_box2d: Option<&Vec<f64>>,
        context: &C,
    ) -> Result<GetAllPlantsResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter/{planter_run_id}/plant",
            self.base_path,
            field_id = utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET),
            planter_run_id = utf8_percent_encode(&param_planter_run_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(param_box2d) = param_box2d {
                query_string.append_pair(
                    "box2d",
                    &param_box2d
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::Plant>>(body)?;
                Ok(GetAllPlantsResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAllPlantsResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAllPlantsResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetAllPlantsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAllPlantsResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_one_plant(
        &self,
        param_field_id: i64,
        param_planter_run_id: i64,
        param_plant_id: i64,
        context: &C,
    ) -> Result<GetOnePlantResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter/{planter_run_id}/plant/{plant_id}",
            self.base_path,
            field_id = utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET),
            planter_run_id = utf8_percent_encode(&param_planter_run_id.to_string(), ID_ENCODE_SET),
            plant_id = utf8_percent_encode(&param_plant_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Plant>(body)?;
                Ok(GetOnePlantResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetOnePlantResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetOnePlantResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetOnePlantResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetOnePlantResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetOnePlantResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn patch_one_plant(
        &self,
        param_field_id: i64,
        param_planter_run_id: i64,
        param_plant_id: i64,
        param_plant_patch: models::PlantPatch,
        context: &C,
    ) -> Result<PatchOnePlantResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter/{planter_run_id}/plant/{plant_id}",
            self.base_path,
            field_id = utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET),
            planter_run_id = utf8_percent_encode(&param_planter_run_id.to_string(), ID_ENCODE_SET),
            plant_id = utf8_percent_encode(&param_plant_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("PATCH")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_plant_patch).expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Plant>(body)?;
                Ok(PatchOnePlantResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PatchOnePlantResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PatchOnePlantResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PatchOnePlantResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(PatchOnePlantResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PatchOnePlantResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_plant(
        &self,
        param_field_id: i64,
        param_planter_run_id: i64,
        param_plant: models::Plant,
        context: &C,
    ) -> Result<PostPlantResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter/{planter_run_id}/plant",
            self.base_path,
            field_id = utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET),
            planter_run_id = utf8_percent_encode(&param_planter_run_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_plant).expect("impossible to fail to serialize");

        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse2013>(body)?;
                Ok(PostPlantResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostPlantResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostPlantResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostPlantResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostPlantResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_planter_run(
        &self,
        param_field_id: i64,
        param_planter_run_id: i64,
        context: &C,
    ) -> Result<DeleteOnePlanterRunResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter/{planter_run_id}",
            self.base_path,
            field_id = utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET),
            planter_run_id = utf8_percent_encode(&param_planter_run_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOnePlanterRunResponse::SuccessfullyDeletedPlanterRun)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOnePlanterRunResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOnePlanterRunResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteOnePlanterRunResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOnePlanterRunResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOnePlanterRunResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_planter_run(
        &self,
        param_field_id: i64,
        context: &C,
    ) -> Result<DeletePlanterRunResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter",
            self.base_path,
            field_id = utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeletePlanterRunResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeletePlanterRunResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeletePlanterRunResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            405 => {
                let body = response.into_body();
                Ok(DeletePlanterRunResponse::MethodNotAllowed)
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_planter_runs(
        &self,
        param_field_id: i64,
        param_box2d: Option<&Vec<f64>>,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllPlanterRunsResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter",
            self.base_path,
            field_id = utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_box2d) = param_box2d {
                query_string.append_pair(
                    "box2d",
                    &param_box2d
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::PlanterRun>>(body)?;
                Ok(GetAllPlanterRunsResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAllPlanterRunsResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAllPlanterRunsResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetAllPlanterRunsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAllPlanterRunsResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_one_planter_run(
        &self,
        param_field_id: i64,
        param_planter_run_id: i64,
        context: &C,
    ) -> Result<GetOnePlanterRunResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter/{planter_run_id}",
            self.base_path,
            field_id = utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET),
            planter_run_id = utf8_percent_encode(&param_planter_run_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::PlanterRun>(body)?;
                Ok(GetOnePlanterRunResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetOnePlanterRunResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetOnePlanterRunResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetOnePlanterRunResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetOnePlanterRunResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetOnePlanterRunResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_planter_run(
        &self,
        param_field_id: i64,
        param_planter_run: models::PlanterRun,
        context: &C,
    ) -> Result<PostPlanterRunResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter",
            self.base_path,
            field_id = utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_planter_run).expect("impossible to fail to serialize");

        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse2012>(body)?;
                Ok(PostPlanterRunResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostPlanterRunResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostPlanterRunResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostPlanterRunResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostPlanterRunResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_quality_measurement(
        &self,
        param_field_id: i64,
        param_planter_run_id: i64,
        param_plant_id: i64,
        param_quality_measurement_id: i64,
        context: &C,
    ) -> Result<DeleteOneQualityMeasurementResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter/{planter_run_id}/plant/{plant_id}/quality_measurement/{quality_measurement_id}",
            self.base_path
            ,field_id=utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET)
            ,planter_run_id=utf8_percent_encode(&param_planter_run_id.to_string(), ID_ENCODE_SET)
            ,plant_id=utf8_percent_encode(&param_plant_id.to_string(), ID_ENCODE_SET)
            ,quality_measurement_id=utf8_percent_encode(&param_quality_measurement_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOneQualityMeasurementResponse::SuccessfullyDeletedQualityMeasurement)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOneQualityMeasurementResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOneQualityMeasurementResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(
                    DeleteOneQualityMeasurementResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                )
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOneQualityMeasurementResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOneQualityMeasurementResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_quality_measurement(
        &self,
        param_field_id: i64,
        param_planter_run_id: i64,
        param_plant_id: i64,
        context: &C,
    ) -> Result<DeleteQualityMeasurementResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter/{planter_run_id}/plant/{plant_id}/quality_measurement",
            self.base_path
            ,field_id=utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET)
            ,planter_run_id=utf8_percent_encode(&param_planter_run_id.to_string(), ID_ENCODE_SET)
            ,plant_id=utf8_percent_encode(&param_plant_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteQualityMeasurementResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteQualityMeasurementResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteQualityMeasurementResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            405 => {
                let body = response.into_body();
                Ok(DeleteQualityMeasurementResponse::MethodNotAllowed)
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_quality_measurements(
        &self,
        param_field_id: i64,
        param_planter_run_id: i64,
        param_plant_id: i64,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllQualityMeasurementsResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter/{planter_run_id}/plant/{plant_id}/quality_measurement",
            self.base_path
            ,field_id=utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET)
            ,planter_run_id=utf8_percent_encode(&param_planter_run_id.to_string(), ID_ENCODE_SET)
            ,plant_id=utf8_percent_encode(&param_plant_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::QualityMeasurement>>(body)?;
                Ok(GetAllQualityMeasurementsResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAllQualityMeasurementsResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAllQualityMeasurementsResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetAllQualityMeasurementsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAllQualityMeasurementsResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_one_quality_measurement(
        &self,
        param_field_id: i64,
        param_planter_run_id: i64,
        param_plant_id: i64,
        param_quality_measurement_id: i64,
        context: &C,
    ) -> Result<GetOneQualityMeasurementResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter/{planter_run_id}/plant/{plant_id}/quality_measurement/{quality_measurement_id}",
            self.base_path
            ,field_id=utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET)
            ,planter_run_id=utf8_percent_encode(&param_planter_run_id.to_string(), ID_ENCODE_SET)
            ,plant_id=utf8_percent_encode(&param_plant_id.to_string(), ID_ENCODE_SET)
            ,quality_measurement_id=utf8_percent_encode(&param_quality_measurement_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::QualityMeasurement>(body)?;
                Ok(GetOneQualityMeasurementResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetOneQualityMeasurementResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetOneQualityMeasurementResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetOneQualityMeasurementResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetOneQualityMeasurementResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetOneQualityMeasurementResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_quality_measurement(
        &self,
        param_field_id: i64,
        param_planter_run_id: i64,
        param_plant_id: i64,
        param_quality_measurement: models::QualityMeasurement,
        context: &C,
    ) -> Result<PostQualityMeasurementResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/field/{field_id}/planter/{planter_run_id}/plant/{plant_id}/quality_measurement",
            self.base_path
            ,field_id=utf8_percent_encode(&param_field_id.to_string(), ID_ENCODE_SET)
            ,planter_run_id=utf8_percent_encode(&param_planter_run_id.to_string(), ID_ENCODE_SET)
            ,plant_id=utf8_percent_encode(&param_plant_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_quality_measurement)
            .expect("impossible to fail to serialize");

        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse2014>(body)?;
                Ok(PostQualityMeasurementResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostQualityMeasurementResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostQualityMeasurementResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostQualityMeasurementResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostQualityMeasurementResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_completed_spray_task(
        &self,
        context: &C,
    ) -> Result<DeleteCompletedSprayTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/spray/complete", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteCompletedSprayTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteCompletedSprayTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteCompletedSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            405 => {
                let body = response.into_body();
                Ok(DeleteCompletedSprayTaskResponse::MethodNotAllowed)
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_completed_spray_task(
        &self,
        param_spray_task_id: i64,
        context: &C,
    ) -> Result<DeleteOneCompletedSprayTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/spray/complete/{spray_task_id}",
            self.base_path,
            spray_task_id = utf8_percent_encode(&param_spray_task_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOneCompletedSprayTaskResponse::SuccessfullyDeletedCompletedSpray)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOneCompletedSprayTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOneCompletedSprayTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(
                    DeleteOneCompletedSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                )
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOneCompletedSprayTaskResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOneCompletedSprayTaskResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_scheduled_spray_task(
        &self,
        param_spray_task_id: i64,
        context: &C,
    ) -> Result<DeleteOneScheduledSprayTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/spray/task/{spray_task_id}",
            self.base_path,
            spray_task_id = utf8_percent_encode(&param_spray_task_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOneScheduledSprayTaskResponse::SuccessfullyDeletedScheduledSprayTask)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOneScheduledSprayTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOneScheduledSprayTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(
                    DeleteOneScheduledSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights
                )
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOneScheduledSprayTaskResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOneScheduledSprayTaskResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_scheduled_spray_task(
        &self,
        context: &C,
    ) -> Result<DeleteScheduledSprayTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/spray/task", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteScheduledSprayTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteScheduledSprayTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteScheduledSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            405 => {
                let body = response.into_body();
                Ok(DeleteScheduledSprayTaskResponse::MethodNotAllowed)
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_completed_spray_tasks(
        &self,
        param_box2d: Option<&Vec<f64>>,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllCompletedSprayTasksResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/spray/complete", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_box2d) = param_box2d {
                query_string.append_pair(
                    "box2d",
                    &param_box2d
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::SprayTask>>(body)?;
                Ok(GetAllCompletedSprayTasksResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAllCompletedSprayTasksResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAllCompletedSprayTasksResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetAllCompletedSprayTasksResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAllCompletedSprayTasksResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_scheduled_spray_tasks(
        &self,
        param_box2d: Option<&Vec<f64>>,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllScheduledSprayTasksResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/spray/task", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_box2d) = param_box2d {
                query_string.append_pair(
                    "box2d",
                    &param_box2d
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::SprayTask>>(body)?;
                Ok(GetAllScheduledSprayTasksResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAllScheduledSprayTasksResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAllScheduledSprayTasksResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetAllScheduledSprayTasksResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAllScheduledSprayTasksResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_one_completed_spray_task(
        &self,
        param_spray_task_id: i64,
        context: &C,
    ) -> Result<GetOneCompletedSprayTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/spray/complete/{spray_task_id}",
            self.base_path,
            spray_task_id = utf8_percent_encode(&param_spray_task_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::SprayTask>(body)?;
                Ok(GetOneCompletedSprayTaskResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetOneCompletedSprayTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetOneCompletedSprayTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetOneCompletedSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetOneCompletedSprayTaskResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetOneCompletedSprayTaskResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_one_scheduled_spray_task(
        &self,
        param_spray_task_id: i64,
        context: &C,
    ) -> Result<GetOneScheduledSprayTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/spray/task/{spray_task_id}",
            self.base_path,
            spray_task_id = utf8_percent_encode(&param_spray_task_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::SprayTask>(body)?;
                Ok(GetOneScheduledSprayTaskResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetOneScheduledSprayTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetOneScheduledSprayTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetOneScheduledSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetOneScheduledSprayTaskResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetOneScheduledSprayTaskResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_completed_spray_task(
        &self,
        param_spray_task: models::SprayTask,
        context: &C,
    ) -> Result<PostCompletedSprayTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/spray/complete", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_spray_task).expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse2016>(body)?;
                Ok(PostCompletedSprayTaskResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostCompletedSprayTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostCompletedSprayTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostCompletedSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostCompletedSprayTaskResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_scheduled_spray_task(
        &self,
        param_spray_task: models::SprayTask,
        context: &C,
    ) -> Result<PostScheduledSprayTaskResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/spray/task", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_spray_task).expect("impossible to fail to serialize");

        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse2016>(body)?;
                Ok(PostScheduledSprayTaskResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostScheduledSprayTaskResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostScheduledSprayTaskResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostScheduledSprayTaskResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostScheduledSprayTaskResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_terrain(
        &self,
        param_terrain_id: i64,
        context: &C,
    ) -> Result<DeleteOneTerrainResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/terrain/{terrain_id}",
            self.base_path,
            terrain_id = utf8_percent_encode(&param_terrain_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOneTerrainResponse::SuccessfullyDeletedTerrainModel)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOneTerrainResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOneTerrainResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteOneTerrainResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOneTerrainResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOneTerrainResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_terrain(&self, context: &C) -> Result<DeleteTerrainResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/terrain", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteTerrainResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteTerrainResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteTerrainResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            405 => {
                let body = response.into_body();
                Ok(DeleteTerrainResponse::MethodNotAllowed)
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_terrains(
        &self,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllTerrainsResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/terrain", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::Terrain>>(body)?;
                Ok(GetAllTerrainsResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAllTerrainsResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAllTerrainsResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetAllTerrainsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAllTerrainsResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_one_terrain(
        &self,
        param_terrain_id: i64,
        context: &C,
    ) -> Result<GetOneTerrainResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/terrain/{terrain_id}",
            self.base_path,
            terrain_id = utf8_percent_encode(&param_terrain_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Terrain>(body)?;
                Ok(GetOneTerrainResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetOneTerrainResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetOneTerrainResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetOneTerrainResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetOneTerrainResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetOneTerrainResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_access_right(
        &self,
        param_user_id: i32,
        param_tokenstr: String,
        param_access_right_path: String,
        context: &C,
    ) -> Result<DeleteOneAccessRightResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/token/{user_id}/{tokenstr}/access_right/{access_right_path}",
            self.base_path,
            user_id = utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET),
            tokenstr = utf8_percent_encode(&param_tokenstr.to_string(), ID_ENCODE_SET),
            access_right_path =
                utf8_percent_encode(&param_access_right_path.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOneAccessRightResponse::SuccessfullyDeletedAccessRight)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOneAccessRightResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOneAccessRightResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteOneAccessRightResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOneAccessRightResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOneAccessRightResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_token(
        &self,
        param_user_id: i32,
        param_tokenstr: String,
        context: &C,
    ) -> Result<DeleteOneTokenResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/token/{user_id}/{tokenstr}",
            self.base_path,
            user_id = utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET),
            tokenstr = utf8_percent_encode(&param_tokenstr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOneTokenResponse::SuccessfullyDeletedToken)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOneTokenResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOneTokenResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteOneTokenResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOneTokenResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOneTokenResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_user(
        &self,
        param_user_id: i32,
        context: &C,
    ) -> Result<DeleteOneUserResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/token/{user_id}",
            self.base_path,
            user_id = utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::Token>>(body)?;
                Ok(DeleteOneUserResponse::SuccessfullyDeletedUser(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOneUserResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOneUserResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteOneUserResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOneUserResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOneUserResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_access_rights(
        &self,
        param_user_id: i32,
        param_tokenstr: String,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetAccessRightsResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/token/{user_id}/{tokenstr}/access_right",
            self.base_path,
            user_id = utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET),
            tokenstr = utf8_percent_encode(&param_tokenstr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::AccessRight>>(body)?;
                Ok(GetAccessRightsResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAccessRightsResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAccessRightsResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetAccessRightsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetAccessRightsResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAccessRightsResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_tokens(
        &self,
        param_tokenstr_contains: Option<&Vec<String>>,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllTokensResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/token", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_tokenstr_contains) = param_tokenstr_contains {
                query_string.append_pair(
                    "tokenstrContains",
                    &param_tokenstr_contains
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::Token>>(body)?;
                Ok(GetAllTokensResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAllTokensResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAllTokensResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetAllTokensResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAllTokensResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_one_access_right(
        &self,
        param_user_id: i32,
        param_tokenstr: String,
        param_access_right_path: String,
        context: &C,
    ) -> Result<GetOneAccessRightResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/token/{user_id}/{tokenstr}/access_right/{access_right_path}",
            self.base_path,
            user_id = utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET),
            tokenstr = utf8_percent_encode(&param_tokenstr.to_string(), ID_ENCODE_SET),
            access_right_path =
                utf8_percent_encode(&param_access_right_path.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::AccessRight>(body)?;
                Ok(GetOneAccessRightResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetOneAccessRightResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetOneAccessRightResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetOneAccessRightResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetOneAccessRightResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetOneAccessRightResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_user_tokens(
        &self,
        param_user_id: i32,
        param_tokenstr_contains: Option<&Vec<String>>,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetUserTokensResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/token/{user_id}",
            self.base_path,
            user_id = utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_tokenstr_contains) = param_tokenstr_contains {
                query_string.append_pair(
                    "tokenstrContains",
                    &param_tokenstr_contains
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::Token>>(body)?;
                Ok(GetUserTokensResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetUserTokensResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetUserTokensResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetUserTokensResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetUserTokensResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetUserTokensResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_access_right(
        &self,
        param_user_id: i32,
        param_tokenstr: String,
        param_access_right: models::AccessRight,
        context: &C,
    ) -> Result<PostAccessRightResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/token/{user_id}/{tokenstr}/access_right",
            self.base_path,
            user_id = utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET),
            tokenstr = utf8_percent_encode(&param_tokenstr.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_access_right).expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::AccessRight>(body)?;
                Ok(PostAccessRightResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostAccessRightResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostAccessRightResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostAccessRightResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(PostAccessRightResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostAccessRightResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_token(
        &self,
        param_token: models::Token,
        context: &C,
    ) -> Result<PostTokenResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/token", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_token).expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::TokenId>(body)?;
                Ok(PostTokenResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostTokenResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostTokenResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostTokenResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostTokenResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_user_token(
        &self,
        param_user_id: i32,
        param_user_token: models::UserToken,
        context: &C,
    ) -> Result<PostUserTokenResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/token/{user_id}",
            self.base_path,
            user_id = utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_user_token).expect("impossible to fail to serialize");

        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::TokenId>(body)?;
                Ok(PostUserTokenResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostUserTokenResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostUserTokenResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostUserTokenResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostUserTokenResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_weather_data(
        &self,
        param_weather_station_id: i32,
        param_measurement_date: String,
        context: &C,
    ) -> Result<DeleteOneWeatherDataResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/weather/data/{weather_station_id}/{measurement_date}",
            self.base_path,
            weather_station_id =
                utf8_percent_encode(&param_weather_station_id.to_string(), ID_ENCODE_SET),
            measurement_date =
                utf8_percent_encode(&param_measurement_date.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOneWeatherDataResponse::SuccessfullyDeletedWeatherData)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOneWeatherDataResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOneWeatherDataResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteOneWeatherDataResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOneWeatherDataResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOneWeatherDataResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_weather_data(
        &self,
        param_station_ids: Option<&Vec<i32>>,
        param_start_date: Option<String>,
        param_end_date: Option<String>,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllWeatherDataResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/weather/data", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_station_ids) = param_station_ids {
                query_string.append_pair(
                    "stationIds",
                    &param_station_ids
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_start_date) = param_start_date {
                query_string.append_pair("startDate", &param_start_date.to_string());
            }
            if let Some(param_end_date) = param_end_date {
                query_string.append_pair("endDate", &param_end_date.to_string());
            }
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::WeatherData>>(body)?;
                Ok(GetAllWeatherDataResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAllWeatherDataResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAllWeatherDataResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetAllWeatherDataResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAllWeatherDataResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_one_weather_data(
        &self,
        param_weather_station_id: i32,
        param_measurement_date: String,
        context: &C,
    ) -> Result<GetOneWeatherDataResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/weather/data/{weather_station_id}/{measurement_date}",
            self.base_path,
            weather_station_id =
                utf8_percent_encode(&param_weather_station_id.to_string(), ID_ENCODE_SET),
            measurement_date =
                utf8_percent_encode(&param_measurement_date.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::WeatherData>(body)?;
                Ok(GetOneWeatherDataResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetOneWeatherDataResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetOneWeatherDataResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetOneWeatherDataResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetOneWeatherDataResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetOneWeatherDataResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn patch_one_weather_data(
        &self,
        param_weather_station_id: i32,
        param_measurement_date: String,
        param_weather_data_patch: models::WeatherDataPatch,
        context: &C,
    ) -> Result<PatchOneWeatherDataResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/weather/data/{weather_station_id}/{measurement_date}",
            self.base_path,
            weather_station_id =
                utf8_percent_encode(&param_weather_station_id.to_string(), ID_ENCODE_SET),
            measurement_date =
                utf8_percent_encode(&param_measurement_date.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("PATCH")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_weather_data_patch)
            .expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::WeatherData>(body)?;
                Ok(PatchOneWeatherDataResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PatchOneWeatherDataResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PatchOneWeatherDataResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PatchOneWeatherDataResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(PatchOneWeatherDataResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PatchOneWeatherDataResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_weather_data(
        &self,
        param_weather_data: models::WeatherData,
        context: &C,
    ) -> Result<PostWeatherDataResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/weather/data", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_weather_data).expect("impossible to fail to serialize");

        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse2019>(body)?;
                Ok(PostWeatherDataResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostWeatherDataResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostWeatherDataResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostWeatherDataResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostWeatherDataResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_one_weather_station(
        &self,
        param_weather_station_id: i32,
        context: &C,
    ) -> Result<DeleteOneWeatherStationResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/weather/station/{weather_station_id}",
            self.base_path,
            weather_station_id =
                utf8_percent_encode(&param_weather_station_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                Ok(DeleteOneWeatherStationResponse::SuccessfullyDeletedWeatherStation)
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(DeleteOneWeatherStationResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(DeleteOneWeatherStationResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(DeleteOneWeatherStationResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(DeleteOneWeatherStationResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(DeleteOneWeatherStationResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_weather_stations(
        &self,
        param_bundesland: Option<&Vec<String>>,
        param_name: Option<&Vec<String>>,
        param_box2d: Option<&Vec<f64>>,
        param_offset: Option<i32>,
        param_limit: Option<i32>,
        context: &C,
    ) -> Result<GetAllWeatherStationsResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/weather/station", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_bundesland) = param_bundesland {
                query_string.append_pair(
                    "bundesland",
                    &param_bundesland
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_name) = param_name {
                query_string.append_pair(
                    "name",
                    &param_name
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_box2d) = param_box2d {
                query_string.append_pair(
                    "box2d",
                    &param_box2d
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            if let Some(param_offset) = param_offset {
                query_string.append_pair("offset", &param_offset.to_string());
            }
            if let Some(param_limit) = param_limit {
                query_string.append_pair("limit", &param_limit.to_string());
            }
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<Vec<models::WeatherStation>>(body)?;
                Ok(GetAllWeatherStationsResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetAllWeatherStationsResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetAllWeatherStationsResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetAllWeatherStationsResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetAllWeatherStationsResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_one_weather_station(
        &self,
        param_weather_station_id: i32,
        context: &C,
    ) -> Result<GetOneWeatherStationResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/weather/station/{weather_station_id}",
            self.base_path,
            weather_station_id =
                utf8_percent_encode(&param_weather_station_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::WeatherStation>(body)?;
                Ok(GetOneWeatherStationResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(GetOneWeatherStationResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(GetOneWeatherStationResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(GetOneWeatherStationResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(GetOneWeatherStationResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(GetOneWeatherStationResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn patch_one_weather_station(
        &self,
        param_weather_station_id: i32,
        param_weather_station_patch: models::WeatherStationPatch,
        context: &C,
    ) -> Result<PatchOneWeatherStationResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/api/v0.3/weather/station/{weather_station_id}",
            self.base_path,
            weather_station_id =
                utf8_percent_encode(&param_weather_station_id.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("PATCH")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_weather_station_patch)
            .expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::WeatherStation>(body)?;
                Ok(PatchOneWeatherStationResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PatchOneWeatherStationResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PatchOneWeatherStationResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PatchOneWeatherStationResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            404 => {
                let body = response.into_body();
                Ok(PatchOneWeatherStationResponse::TheSpecifiedResourceWasNotFound)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PatchOneWeatherStationResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn post_weather_station(
        &self,
        param_weather_station: models::WeatherStation,
        context: &C,
    ) -> Result<PostWeatherStationResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/api/v0.3/weather/station", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(auth_data) = (context as &dyn Has<Option<AuthData>>).get().as_ref() {
                if let AuthData::ApiKey(ref api_key) = *auth_data {
                    query_string.append_pair("api-token", api_key);
                }
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_weather_station).expect("impossible to fail to serialize");

        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );

        let header = HeaderValue::from_str(
            Has::<XSpanIdString>::get(context)
                .0
                .clone()
                .to_string()
                .as_str(),
        );
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            match auth_data {
                _ => {}
            }
        }

        let mut response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            201 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse2018>(body)?;
                Ok(PostWeatherStationResponse::SuccessfulOperation(body))
            }
            400 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::InlineResponse400>(body)?;
                Ok(PostWeatherStationResponse::BadRequest(body))
            }
            401 => {
                let body = response.into_body();
                Ok(PostWeatherStationResponse::MissingAPIKey)
            }
            403 => {
                let body = response.into_body();
                Ok(PostWeatherStationResponse::SpecifiedAPIKeyDoesNotHaveTheRequiredRights)
            }
            500 => {
                let body = response.into_body();
                let body = body
                    .to_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Error>(body)?;
                Ok(PostWeatherStationResponse::BackendError(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).to_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }
}
