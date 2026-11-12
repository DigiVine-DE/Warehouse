#![feature(generic_associated_types)]

extern crate diesel;
extern crate diesel_migrations;
extern crate frunk_core;
extern crate openssl;
#[macro_use]
extern crate frunk;
extern crate log;

pub mod field;
pub mod forecast;
pub mod harvest;
pub mod quality_measurement;
mod schema;
pub mod spray;
pub mod weather;

use diesel_ext::db_backend::{Backend, DatabaseError};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RelationalBackendLibError {
    #[error("error during database communication: {0}")]
    DBError(#[from] DatabaseError),
    #[error("could not deserialize {field}: {reason}")]
    DeserializationError { field: String, reason: String },
}

pub struct Relational;
impl Backend for Relational {
    type Error = RelationalBackendLibError;

    const NAME: &'static str = "Relational";
    const SOURCE: EmbeddedMigrations = embed_migrations!();
    const MOCK_DATA: Option<EmbeddedMigrations> = Some(embed_migrations!("mockdata"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn migration_test() {
        Relational::db_tester();
    }
}
