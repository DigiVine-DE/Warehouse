extern crate openssl;
#[macro_use]
extern crate diesel;

use diesel::{pg::PgConnection, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel_ext::{db_tester::DbTester, GeoBBox2D};
use postgis_diesel::gps::{MultiPoint, Point};

#[test]
fn from_to_sql() {
    db_test(|conn| {
        Tests::new(
            "Single Point",
            vec![Point::new(1., 0.)],
            [1., 0., 1., 0.].into(),
        )
        .check(conn);

        Tests::new(
            "Line String",
            vec![Point::new(1., 0.), Point::new(1., 2.)],
            [1., 0., 1., 2.].into(),
        )
        .check(conn);

        Tests::new(
            "Polygon",
            vec![Point::new(1., 0.), Point::new(0., 1.)],
            [0., 0., 1., 1.].into(),
        )
        .check(conn);
    })
}

table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;
    geo_bbox_test (id) {
        id -> Int4,
        multi_points -> Geometry,
        bbox -> Nullable<Geometry>,
    }
}

#[derive(Insertable)]
#[diesel(table_name = geo_bbox_test)]
struct GeoBboxTestInsert {
    multi_points: MultiPoint,
    bbox: Option<GeoBBox2D>,
}

struct Tests {
    name: &'static str,
    points: Vec<Point>,
    want_bbox: GeoBBox2D,
}

impl Tests {
    fn new(name: &'static str, points: Vec<Point>, want_bbox: GeoBBox2D) -> Self {
        Self {
            name,
            points,
            want_bbox,
        }
    }

    fn check(&self, conn: &mut PgConnection) {
        let want = GeoBboxTestInsert::new(self.points.to_owned());
        let id = want.insert(conn);
        let got = GeoBboxTest::get(conn, id);
        assert_eq!(got.multi_points, want.multi_points, "Test: {}", self.name);
        assert_eq!(got.bbox, Some(self.want_bbox), "Test: {}", self.name);
    }
}

impl GeoBboxTestInsert {
    fn new(points: Vec<Point>) -> Self {
        Self {
            multi_points: points.into_iter().collect(),
            bbox: None,
        }
    }

    fn insert(&self, conn: &mut PgConnection) -> i32 {
        let point_from_db: GeoBboxTest = diesel::insert_into(geo_bbox_test::table)
            .values(self)
            .get_result(conn)
            .expect("Error saving GeoBboxTest");
        diesel::sql_query("UPDATE geo_bbox_test SET bbox = st_envelope(multi_points)")
            .execute(conn)
            .unwrap();
        point_from_db.id
    }
}

#[derive(Queryable, Debug, PartialEq)]
#[diesel(table_name = geo_bbox_test)]
struct GeoBboxTest {
    id: i32,
    multi_points: MultiPoint,
    bbox: Option<GeoBBox2D>,
}

impl GeoBboxTest {
    fn get(conn: &mut PgConnection, id: i32) -> Self {
        geo_bbox_test::table
            .filter(geo_bbox_test::id.eq(id))
            .first(conn)
            .unwrap()
    }
}

fn establish_connection() -> PgConnection {
    let database_url = DbTester::test_url();

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn initialize() -> PgConnection {
    let mut conn = establish_connection();
    diesel::sql_query("CREATE EXTENSION IF NOT EXISTS postgis")
        .execute(&mut conn)
        .unwrap();
    drop_table(&mut conn);

    diesel::sql_query(
        r#"
            CREATE TABLE geo_bbox_test
(
    id                SERIAL PRIMARY KEY,
    multi_points      geometry(MultiPoint,4326) NOT NULL,
    bbox              geometry
)
"#,
    )
    .execute(&mut conn)
    .unwrap();
    conn
}

fn drop_table(conn: &mut PgConnection) {
    diesel::sql_query("DROP TABLE IF EXISTS geo_bbox_test")
        .execute(conn)
        .unwrap();
}

fn db_test<F: Fn(&mut PgConnection)>(test_fn: F) {
    let mut conn = initialize();
    test_fn(&mut conn);
    drop_table(&mut conn);
}
