use super::{NestedSprayTask, SprayPlantProtectionInsert};
use crate::{
    diesel::prelude::*,
    schema::{spraytask::dsl, *},
    spray::{SprayPlantProtectionPersistent, SprayTaskPersistent},
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

impl<Conn> DbEntry<Conn> for NestedSprayTask {
    type B = Relational;
    type ID = i64;
}

#[gen_read_tests(id = self.spray.task_id, testable_ids = 1..19)]
impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for NestedSprayTask {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        let entry = match dsl::spraytask.find(id).first::<SprayTaskPersistent>(c) {
            Ok(e) => e,
            Err(e) => match e {
                diesel::result::Error::NotFound => {
                    return Ok(None);
                }
                _ => return Err(DatabaseError::OperationError(e)),
            },
        };
        let protections = SprayPlantProtectionPersistent::belonging_to(&entry)
            .load::<SprayPlantProtectionPersistent>(c)
            .map_err(DatabaseError::from)?;

        Ok(Some(NestedSprayTask {
            spray: entry,
            protections,
        }))
    }
}

#[gen_batch_read_tests(limit = 18)]
impl<Conn: LoadConnection<Backend = Pg>> Searchable<Conn> for NestedSprayTask {
    type Filter<'a> = Option<GeoBBox2D>;

    fn search(
        c: &mut Conn,
        box2d: Self::Filter<'_>,
        offset: usize,
        count: usize,
    ) -> Result<Vec<Self>, DatabaseError> {
        use postgis_diesel::operators::intersects_2d;
        let mut query = dsl::spraytask
            .offset(offset as i64)
            .limit(count as i64)
            .into_boxed();
        if let Some(bbox) = box2d {
            query = query.filter(intersects_2d(dsl::geom, bbox));
        }
        let entries = query
            .load::<SprayTaskPersistent>(c)
            .map_err(DatabaseError::from)?;
        let protections = SprayPlantProtectionPersistent::belonging_to(&entries)
            .load::<SprayPlantProtectionPersistent>(c)
            .map_err(DatabaseError::from)?
            .grouped_by(&entries);

        let result = entries.into_iter().zip(protections);

        Ok(result
            .map(|(h, nm)| NestedSprayTask {
                spray: h,
                protections: nm,
            })
            .collect())
    }
}

#[gen_delete_tests]
impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for NestedSprayTask {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        diesel::delete(dsl::spraytask.filter(dsl::task_id.eq(id)))
            .returning(dsl::task_id)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

#[derive(PartialEq, Debug, Clone, Insertable)]
#[diesel(table_name = spraytask)]
pub struct SprayTaskInsert {
    pub task_name: String,
    pub customer_name: String,
    pub farm_name: Option<String>,
    pub fruit_group_name: String,
    pub fruit_name: String,
    pub plain_name: String,
    pub plain_size: i64,
    pub geom: Polygon,
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Mockable))]
pub struct NestedSprayTaskInsert {
    pub spray: SprayTaskPersistent,
    pub protections: Vec<SprayPlantProtectionInsert>,
}

#[gen_create_tests]
impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, NestedSprayTask>
    for NestedSprayTaskInsert
{
    fn create(mut self, c: &mut Conn) -> Result<i64, DatabaseError> {
        use crate::schema::spray_plantprotection::dsl::spray_plantprotection;

        c.transaction::<_, DatabaseError, _>(|conn| {
            //Insert top level entry
            let spray_id = diesel::insert_into(dsl::spraytask)
                .values(&self.spray)
                .get_result::<SprayTaskPersistent>(conn)?
                .task_id;

            for p in &mut self.protections {
                p.task_id = Some(spray_id);
            }

            // Insert protections
            diesel::insert_into(spray_plantprotection)
                .values(self.protections)
                .execute(conn)?;
            Ok(spray_id)
        })
    }
}
