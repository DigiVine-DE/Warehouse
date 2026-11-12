#![feature(generic_associated_types)]

extern crate openssl;
#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

pub mod db;
mod filter;
mod schema;

use diesel_ext::db_backend::{Backend, DatabaseError};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
pub use filter::TokenFilter;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthLibError {
    #[error("error during database communication: {0}")]
    DBError(#[from] DatabaseError),
}

pub struct Auth;
impl Backend for Auth {
    type Error = AuthLibError;

    const NAME: &'static str = "Auth";
    const SOURCE: EmbeddedMigrations = embed_migrations!();
    const MOCK_DATA: Option<EmbeddedMigrations> = None;
}
