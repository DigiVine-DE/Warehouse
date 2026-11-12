use auth_backend::Auth;
use diesel_ext::db_backend::Backend;
use digivine_api_ero::Ero;
use digivine_api_jki_lidar::JkiLidar;
use digivine_api_jki_shape::JkiShape;
use digivine_api_postgres::Relational;
use digivine_api_terrain::Terrain;
use serial_test::serial;

/*

   _____  ____    __  __ _                 _   _
  |  __ \|  _ \  |  \/  (_)               | | (_)
  | |  | | |_) | | \  / |_  __ _ _ __ __ _| |_ _  ___  _ __  ___
  | |  | |  _ <  | |\/| | |/ _` | '__/ _` | __| |/ _ \| '_ \/ __|
  | |__| | |_) | | |  | | | (_| | | | (_| | |_| | (_) | | | \__ \
  |_____/|____/  |_|  |_|_|\__, |_|  \__,_|\__|_|\___/|_| |_|___/
                            __/ |
                           |___/

*/

#[test]
#[ignore]
#[serial]
fn test_db_auth_migration() {
    Auth::db_tester();
}

#[test]
#[ignore]
#[serial]
fn test_db_ero_migration() {
    Ero::db_tester();
}

#[test]
#[ignore]
#[serial]
fn test_db_jki_lidar_migration() {
    JkiLidar::db_tester();
}

#[test]
#[ignore]
#[serial]
fn test_db_jki_shape_migration() {
    JkiShape::db_tester();
}

#[test]
#[ignore]
#[serial]
fn test_db_postgres_migration() {
    Relational::db_tester();
}

#[test]
#[ignore]
#[serial]
fn test_db_terrain_migration() {
    Terrain::db_tester();
}
