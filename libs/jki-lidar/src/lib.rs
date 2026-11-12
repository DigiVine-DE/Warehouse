extern crate openssl;
#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
extern crate log;

pub mod db;
mod schema;

use diesel_ext::db_backend::{Backend, DatabaseError};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

use thiserror::Error;

pub struct JkiLidar;
impl Backend for JkiLidar {
    type Error = JkiLidarLibError;

    const NAME: &'static str = "Jki Lidar";
    const SOURCE: EmbeddedMigrations = embed_migrations!();
    const MOCK_DATA: Option<EmbeddedMigrations> = None;
}

#[derive(Error, Debug)]
pub enum JkiLidarLibError {
    #[error("error during database communication: {0}")]
    DBError(#[from] DatabaseError),
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[ignore]
//     fn migration_test() {
//         JkiLidar::db_tester();
//     }

//     #[test]
//     fn jki_lidar_test() {
//         let mut c = JkiLidar::db_tester().test_connection();
//         let all = c.get_all(0, 10).unwrap();
//         for e in all {
//             let id = e.id.unwrap();
//             assert_eq!(e, c.get(id).unwrap().unwrap());
//             assert_eq!(e, c.remove(id).unwrap().unwrap());
//             assert!(c.get(id).unwrap().is_none());
//         }
//     }
// }
