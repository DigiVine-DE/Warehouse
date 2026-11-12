extern crate openssl;
#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
extern crate log;

use diesel_ext::db_backend::{Backend, DatabaseError};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

use thiserror::Error;

pub mod db;
mod schema;

pub struct Ero;
impl Backend for Ero {
    type Error = EroLibError;

    const NAME: &'static str = "Ero";
    const SOURCE: EmbeddedMigrations = embed_migrations!();
    const MOCK_DATA: Option<EmbeddedMigrations> = None;
}

#[derive(Error, Debug)]
pub enum EroLibError {
    #[error("error during database communication: {0}")]
    DBError(#[from] DatabaseError),
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     #[ignore]
//     fn migration_test() {
//         Ero::db_tester();
//     }

//     #[test]
//     fn ero_test() {
//         let mut c = Ero::db_tester().test_connection();
//         let all = c.get_all(0, 10).unwrap();
//         assert_eq!(all.len(), 10);
//         for e in all {
//             let id = e.id.unwrap();
//             assert_eq!(e, c.get(id).unwrap().unwrap());
//             assert_eq!(e, c.remove(id).unwrap().unwrap());
//             assert!(c.get(id).unwrap().is_none());
//         }
//     }
// }
