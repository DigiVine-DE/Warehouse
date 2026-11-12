use config::{Config, ConfigError, Environment, File};
use diesel_ext::db_backend::{Backend, DbConnector};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    // Database config
    pub db: Database,
    // If database should be initialized with mockdata
    pub mockdata: bool,
    pub server_addr: String,
    pub log_level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseModules {
    pub ero: String,
    pub jki_lidar: String,
    pub jki_shape: String,
    pub forecast: String,
    pub wagner: String,
    pub auth: String,
    pub terrain: String,
    pub relational: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseGlobal {
    pub connect: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Database {
    #[serde(rename = "global")]
    Global(String),
    #[serde(rename = "modules")]
    Module(DatabaseModules),
}

impl Settings {
    pub fn init_backend<B: Backend, Conn: DbConnector>(&self, _b: B) -> Result<Conn, B::Error> {
        let db_url = match &self.db {
            Database::Global(db) => db,
            Database::Module(dbs) => match B::NAME {
                "Auth" => &dbs.auth,
                "Ero" => &dbs.ero,
                "Jki Lidar" => &dbs.jki_lidar,
                "Jki Shape" => &dbs.jki_shape,
                "Relational" => &dbs.relational,
                "Terrain" => &dbs.terrain,
                _ => panic!("unsupported backend: {}", B::NAME),
            },
        };
        B::new(db_url, self.mockdata)
    }

    pub fn new() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            .set_default("log_level", "Debug")?
            .set_default("server_addr", "127.0.0.1:8080")?
            .set_default("mockdata", false)?
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("conf_default").required(false))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("conf").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("digivine").separator("_"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        builder.try_deserialize()
    }
}
