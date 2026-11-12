use super::auth::MakeAllowAPIAuthenticator;
use crate::{config::Settings, ApplicationError};
use auth_backend::AuthLibError;
use diesel_ext::db_backend::{Backend, DbConnector};
use digivine_api_ero::EroLibError;
use digivine_api_jki_lidar::JkiLidarLibError;
use digivine_api_jki_shape::JkiShapeLibError;
use digivine_api_postgres::RelationalBackendLibError;
use digivine_api_terrain::TerrainLibError;
use digivine_api_vivc::VivcLibError;
use digivine_server::server::{context::MakeAddContext, MakeService};
use std::{
    marker::PhantomData,
    option::Option,
    sync::{Arc, Mutex},
};
use swagger::EmptyContext;
use tokio::sync::oneshot;

#[derive(Clone)]
pub struct Backends<Conn: DbConnector> {
    pub ero_backend: Arc<Mutex<Conn>>,
    pub jki_lidar_backend: Arc<Mutex<Conn>>,
    pub jki_shape_backend: Arc<Mutex<Conn>>,
    pub relational_backend: Arc<Mutex<Conn>>,
    pub terrain_backend: Arc<Mutex<Conn>>,
    pub auth_backend: Arc<Mutex<Conn>>,
    pub vivc_backend: Arc<Mutex<Conn>>,
}

impl<Conn: DbConnector> Backends<Conn> {
    pub fn init(
        conf: &Settings,
        e: impl Backend<Error = EroLibError>,
        jl: impl Backend<Error = JkiLidarLibError>,
        js: impl Backend<Error = JkiShapeLibError>,
        r: impl Backend<Error = RelationalBackendLibError>,
        t: impl Backend<Error = TerrainLibError>,
        a: impl Backend<Error = AuthLibError>,
        v: impl Backend<Error = VivcLibError>,
    ) -> Result<Self, ApplicationError> {
        let be = Backends {
            ero_backend: Arc::new(Mutex::new(conf.init_backend(e)?)),
            jki_lidar_backend: Arc::new(Mutex::new(conf.init_backend(jl)?)),
            jki_shape_backend: Arc::new(Mutex::new(conf.init_backend(js)?)),
            relational_backend: Arc::new(Mutex::new(conf.init_backend(r)?)),
            terrain_backend: Arc::new(Mutex::new(conf.init_backend(t)?)),
            auth_backend: Arc::new(Mutex::new(conf.init_backend(a)?)),
            vivc_backend: Arc::new(Mutex::new(conf.init_backend(v)?)),
        };
        Ok(be)
    }

    /// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
    pub async fn start_server(self, addr: &str, shutdown_signal: Option<oneshot::Receiver<()>>) {
        let auth_backend = Arc::clone(&self.auth_backend);

        let server = Server::new(self);
        let service = MakeService::new(server);
        let service = MakeAllowAPIAuthenticator::new(service, Arc::clone(&auth_backend));
        let service = MakeAddContext::<_, EmptyContext>::new(service);

        let addr = addr.parse().expect("Failed to parse bind address");
        let s = hyper::Server::bind(&addr).serve(service);
        match shutdown_signal {
            Some(signal) => s
                .with_graceful_shutdown(async {
                    signal.await.ok();
                })
                .await
                .unwrap(),
            None => s.await.unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct Server<C, Conn: DbConnector> {
    marker: PhantomData<C>,
    pub backends: Backends<Conn>,
}

impl<C, Conn: DbConnector> Server<C, Conn> {
    pub fn new(backends: Backends<Conn>) -> Self {
        Self {
            marker: PhantomData,
            backends,
        }
    }
}
