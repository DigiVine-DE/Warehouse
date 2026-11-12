extern crate diesel;
extern crate diesel_migrations;
extern crate openssl;

pub mod db;
mod schema;

pub use db::NestedTerrain;
use diesel_ext::db_backend::{Backend, DatabaseError};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use thiserror::Error;

pub struct Terrain;
impl Backend for Terrain {
    type Error = TerrainLibError;

    const NAME: &'static str = "Terrain";
    const SOURCE: EmbeddedMigrations = embed_migrations!();
    const MOCK_DATA: Option<EmbeddedMigrations> = Some(embed_migrations!("mockdata"));
}

#[derive(Error, Debug)]
pub enum TerrainLibError {
    #[error("error during database communication: {0}")]
    DBError(#[from] DatabaseError),
}
