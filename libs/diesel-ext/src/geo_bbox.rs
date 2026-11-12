use postgis_diesel::{
    gps::{GeometryContainer, MultiPolygon, Point, Polygon},
    sql_types::Geometry,
};
use std::fmt::Debug;

use diesel::{
    deserialize::{self, FromSql},
    pg::{self, Pg},
    serialize::{self, Output, ToSql},
};

#[derive(Copy, Clone, Debug, FromSqlRow, AsExpression, PartialEq, Default)]
#[diesel(sql_type = Geometry)]
pub struct GeoBBox2D([f64; 4]);

fn conversion_error<S: Debug>(s: S) -> String {
    format!("BBox must have 4 elements: {:?}", s)
}

fn vec_to_array<T: Debug, const N: usize>(v: Vec<T>) -> Result<[T; N], String> {
    v.try_into().map_err(conversion_error)
}

impl FromSql<Geometry, Pg> for GeoBBox2D {
    fn from_sql(bytes: pg::PgValue) -> deserialize::Result<Self> {
        let geom = GeometryContainer::from_sql(bytes)?;
        let bbox = match geom {
            GeometryContainer::Point(p) => [p.x, p.y, p.x, p.y],
            GeometryContainer::LineString(l) => {
                let [min, max] = vec_to_array(l.points)?;
                [min.x, min.y, max.x, max.y]
            }
            GeometryContainer::Polygon(p) => {
                let [ring] = vec_to_array(p.rings)?;
                let [min, _, max, _, _] = vec_to_array(ring.points)?;
                [min.x, min.y, max.x, max.y]
            }
            _ => {
                return deserialize::Result::Err(
                    format!(
                        "a geometry bbox only supports Point, LineString or Polygon: {:?}",
                        geom
                    )
                    .into(),
                );
            }
        };
        Ok(bbox.into())
    }
}

impl ToSql<Geometry, Pg> for GeoBBox2D {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let g = self.to_geo();
        g.to_sql(&mut out.reborrow())
    }
}

impl From<[f64; 4]> for GeoBBox2D {
    fn from(a: [f64; 4]) -> Self {
        Self(a)
    }
}

impl TryFrom<Vec<f64>> for GeoBBox2D {
    type Error = String;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        let a = vec_to_array(value)?;
        Ok(a.into())
    }
}

impl TryFrom<&Vec<f64>> for GeoBBox2D {
    type Error = String;

    fn try_from(value: &Vec<f64>) -> Result<Self, Self::Error> {
        let a: [f64; 4] = value.as_slice().try_into().map_err(conversion_error)?;
        Ok(a.into())
    }
}

impl From<GeoBBox2D> for Vec<f64> {
    fn from(val: GeoBBox2D) -> Self {
        val.0.into()
    }
}

fn new_polygon(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Polygon {
    let line_string = vec![
        Point::new(min_x, min_y),
        Point::new(min_x, max_y),
        Point::new(max_x, max_y),
        Point::new(max_x, min_y),
        Point::new(min_x, min_y),
    ]
    .into_iter()
    .collect();

    vec![line_string].into_iter().collect()
}

impl GeoBBox2D {
    pub fn to_geo(&self) -> GeometryContainer {
        let [min_x, min_y, max_x, max_y] = self.0;
        match (min_x == max_x, min_y == max_y) {
            (true, true) => GeometryContainer::Point(Point::new(min_x, min_y)),
            (false, false) => GeometryContainer::Polygon(new_polygon(min_x, min_y, max_x, max_y)),
            _ => GeometryContainer::LineString(
                [Point::new(min_x, min_y), Point::new(max_x, max_y)]
                    .into_iter()
                    .collect(),
            ),
        }
    }

    pub fn to_multi_polygon(&self) -> MultiPolygon {
        let [min_x, min_y, max_x, max_y] = self.0;
        let poly = new_polygon(min_x, min_y, max_x, max_y);
        vec![poly].into_iter().collect()
    }
}
