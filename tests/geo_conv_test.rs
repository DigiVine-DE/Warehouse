use diesel_ext::db_backend::{Backend, Searchable};
use digivine_api_postgres::{field::db::FieldPersistent, Relational};
use serial_test::serial;

#[test]
#[ignore]
#[serial]
fn test_db_postgres_geo_conv() {
    use digivine_api::geo_conv::*;
    let mut conn = Relational::db_tester().test_connection();
    let fields: Vec<_> = FieldPersistent::search(&mut conn, Default::default(), 0, 100)
        .unwrap()
        .into_iter()
        .flat_map(|nf| nf.geom)
        .collect();

    let converted: Vec<_> = fields
        .clone()
        .into_iter()
        .map(|x| x.try_to_json().unwrap().try_to_diesel().unwrap())
        .collect();

    assert_eq!(fields, converted)
}
