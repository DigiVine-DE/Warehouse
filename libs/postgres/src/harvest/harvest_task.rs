use crate::{
    diesel::prelude::*,
    schema::{harvesttask::dsl, *},
    DatabaseError, Relational,
};
use diesel::{connection::LoadConnection, pg::Pg};
use diesel_ext::{
    db_backend::{Creatable, DbEntry, Deletable, Readable, Searchable},
    GeoBBox2D,
};
#[cfg(test)]
use macros_lib::Mockable;
use macros_lib::{gen_batch_read_tests, gen_create_tests, gen_delete_tests, gen_read_tests};
use postgis_diesel::gps::*;

#[derive(PartialEq, Debug, Clone, Identifiable, Queryable, Insertable, AsChangeset)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(primary_key(task_id))]
#[diesel(table_name = harvesttask)]
pub struct HarvestTaskPersistent {
    pub task_id: i64,
    pub task_name: String,
    pub customer_name: String,
    pub farm_name: Option<String>,
    pub fruit_group_name: String,
    pub fruit_name: String,
    pub plain_name: String,
    pub plain_size: i64,
    pub yield_total_mass: Option<i64>,
    pub total_area: Option<i64>,
    pub effective_total_distance: Option<i64>,
    pub effective_total_time: Option<i64>,
    pub geom: Polygon,
}

impl<Conn> DbEntry<Conn> for HarvestTaskPersistent {
    type B = Relational;
    type ID = i64;
}

#[gen_read_tests(id = self.task_id, testable_ids = 1..19)]
impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for HarvestTaskPersistent {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        match dsl::harvesttask.find(id).first::<HarvestTaskPersistent>(c) {
            Ok(e) => Ok(Some(e)),
            Err(e) => match e {
                diesel::result::Error::NotFound => Ok(None),
                _ => Err(DatabaseError::OperationError(e)),
            },
        }
    }
}

#[gen_batch_read_tests(limit = 18)]
impl<Conn: LoadConnection<Backend = Pg>> Searchable<Conn> for HarvestTaskPersistent {
    type Filter<'a> = Option<GeoBBox2D>;

    fn search(
        c: &mut Conn,
        box2d: Self::Filter<'_>,
        offset: usize,
        count: usize,
    ) -> Result<Vec<Self>, DatabaseError> {
        use postgis_diesel::operators::intersects_2d;
        let mut query = dsl::harvesttask
            .offset(offset as i64)
            .limit(count as i64)
            .into_boxed();
        if let Some(bbox) = box2d {
            query = query.filter(intersects_2d(dsl::geom, bbox));
        }
        let entries = query
            .load::<HarvestTaskPersistent>(c)
            .map_err(DatabaseError::from)?;

        Ok(entries)
    }
}

#[gen_delete_tests]
impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for HarvestTaskPersistent {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        diesel::delete(dsl::harvesttask.filter(dsl::task_id.eq(id)))
            .returning(dsl::task_id)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

#[gen_create_tests]
impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, Self> for HarvestTaskPersistent {
    fn create(self, c: &mut Conn) -> Result<i64, DatabaseError> {
        diesel::insert_into(dsl::harvesttask)
            .values(self)
            .returning(dsl::task_id)
            .get_result(c)
            .map_err(DatabaseError::from)
    }
}
