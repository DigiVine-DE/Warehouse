extern crate digivine_api_ero;
extern crate digivine_api_jki_lidar;
extern crate digivine_api_jki_shape;
extern crate digivine_api_postgres;
extern crate digivine_api_terrain;

use crate::server::Backends;
use auth_backend::Auth;
use diesel_ext::db_backend::{DbConnector, DbConnectorInitError};
use digivine_api_ero::Ero;
use digivine_api_jki_lidar::JkiLidar;
use digivine_api_jki_shape::JkiShape;
use digivine_api_postgres::Relational;
use digivine_api_terrain::Terrain;
use digivine_api_vivc::Vivc;
use std::str::FromStr;
use tokio::sync::oneshot;

pub mod api;
pub mod auth;
pub mod config;
pub mod field;
pub mod forecast;
pub mod geo_conv;
pub mod harvest;
pub mod quality_measurement;
pub mod response;
pub mod server;
pub mod spray;
pub mod terrain;
pub mod vivc;
pub mod weather;

#[macro_use]
extern crate thiserror;

#[macro_use]
extern crate log;
extern crate simplelog;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("error initializing backend: {0}")]
    InitBackendError(#[from] DbConnectorInitError),
    #[error("error during communication with Auth DB: {0}")]
    AuthDBError(#[from] auth_backend::AuthLibError),
    #[error("error during communication with Ero DB: {0}")]
    EroDBError(#[from] digivine_api_ero::EroLibError),
    #[error("error during communication with JKI Lidar DB: {0}")]
    JkiLidarDBError(#[from] digivine_api_jki_lidar::JkiLidarLibError),
    #[error("error during communication with JKI Shape DB: {0}")]
    JkiShapeDBError(#[from] digivine_api_jki_shape::JkiShapeLibError),
    #[error("error during communication with Postgres DB: {0}")]
    RelationalBackendDBError(#[from] digivine_api_postgres::RelationalBackendLibError),
    #[error("error during communication with Terrain DB: {0}")]
    TerrainBackendDBError(#[from] digivine_api_terrain::TerrainLibError),
    #[error("error during communication with Vivc DB: {0}")]
    VivcBackendDBError(#[from] digivine_api_vivc::VivcLibError),
}

pub fn init_log(conf: &config::Settings) {
    let mut log_config_builder = simplelog::ConfigBuilder::new();
    log_config_builder.add_filter_ignore_str("hyper::proto");
    let level_filter = simplelog::LevelFilter::from_str(&conf.log_level).expect("Log filter level cannot be deserialized, possible values are: Off, Error, Warn, Info, Debug, Trace,");
    simplelog::SimpleLogger::init(level_filter, log_config_builder.build())
        .expect("could not init log");
    info!("Set log filter level as [{}]", level_filter);
}

/// Prepares the Rocket HTTP server
pub async fn start_server<Conn: DbConnector>(
    conf: &config::Settings,
    shutdown_signal: Option<oneshot::Receiver<()>>,
) -> Result<(), ApplicationError> {
    if cfg!(debug_assertions) {
        warn!("Running in debug mode, remember to use the libs/reset_database.sh script to update the schema.rs files if migrations have been changed.")
    }
    let addr = &conf.server_addr;
    info!("Starting server at {}", addr);
    Backends::<Conn>::init(
        conf, Ero, JkiLidar, JkiShape, Relational, Terrain, Auth, Vivc,
    )?
    .start_server(addr, shutdown_signal)
    .await;
    Ok(())
}
