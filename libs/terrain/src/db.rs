use super::schema::{terrain::dsl, *};
use crate::{diesel::prelude::*, Terrain};
use diesel::{connection::LoadConnection, pg::Pg};
use diesel_ext::db_backend::{BatchReadable, DatabaseError, DbEntry, Deletable, Readable};
use macros_lib::{gen_batch_read_tests, gen_delete_tests, gen_read_tests};
use postgis_diesel::gps::PointZ;

//////////////// Queries ////////////////
#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = terrain)]
pub struct TerrainPersistent {
    pub id: i64,
    pub description: String,
    pub field_id: Option<i64>,
}

#[derive(Identifiable, Queryable, Insertable, Associations, PartialEq, Debug)]
#[diesel(primary_key(id, x, y))]
#[diesel(belongs_to(TerrainPersistent, foreign_key = id))]
#[diesel(table_name = terrain_point)]
pub struct TerrainPointPersistent {
    pub id: i64,
    pub x: i32,
    pub y: i32,
    pub geo_point_z: PointZ,
}

#[derive(Debug, PartialEq)]
pub struct NestedTerrain {
    pub meta: TerrainPersistent,
    pub data: Vec<TerrainPointPersistent>,
}

impl<Conn: LoadConnection<Backend = Pg>> DbEntry<Conn> for NestedTerrain {
    type B = Terrain;
    type ID = i64;
}

#[gen_read_tests(id = self.meta.id, testable_ids = 1)]
impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for NestedTerrain {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        let entry = match dsl::terrain.find(id).first::<TerrainPersistent>(c) {
            Ok(e) => e,
            Err(e) => match e {
                diesel::result::Error::NotFound => {
                    return Ok(None);
                }
                _ => return Err(DatabaseError::OperationError(e)),
            },
        };
        let points = TerrainPointPersistent::belonging_to(&entry)
            .load::<TerrainPointPersistent>(c)
            .map_err(DatabaseError::from)?;
        let data = NestedTerrain {
            meta: entry,
            data: points,
        };
        Ok(Some(data))
    }
}

#[gen_batch_read_tests(limit = 1)]
impl<Conn: LoadConnection<Backend = Pg>> BatchReadable<Conn> for NestedTerrain {
    fn read_all(c: &mut Conn, offset: usize, count: usize) -> Result<Vec<Self>, DatabaseError> {
        let meta = dsl::terrain
            .offset(offset as i64)
            .limit(count as i64)
            .load::<TerrainPersistent>(c)
            .map_err(DatabaseError::from)?;

        let points = TerrainPointPersistent::belonging_to(&meta)
            .load::<TerrainPointPersistent>(c)
            .map_err(DatabaseError::from)?
            .grouped_by(&meta);

        let data = meta
            .into_iter()
            .zip(points)
            .map(|(e, p)| NestedTerrain { meta: e, data: p })
            .collect::<Vec<_>>();
        Ok(data)
    }
}

#[gen_delete_tests]
impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for NestedTerrain {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        use super::schema::{terrain::dsl as te, terrain_point::dsl as tp};

        diesel::delete(tp::terrain_point)
            .filter(tp::id.eq(id))
            .execute(c)
            .map_err(DatabaseError::from)?;

        diesel::delete(te::terrain.filter(te::id.eq(id)))
            .returning(te::id)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}
