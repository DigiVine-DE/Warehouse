use super::plant::db::{PlantInsert, PlantPersistent};
use crate::{
    diesel::prelude::*,
    field::db::FieldPersistent,
    schema::{planting_run::dsl, *},
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

//////////////// Queries ////////////////
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[diesel(belongs_to(FieldPersistent, foreign_key = field_id))]
#[diesel(table_name = planting_run)]
pub struct PlantingRunPersistent {
    pub id: i64,
    pub user_id: i64,
    pub terminal_id: i64,
    pub field_id: i64,
    pub bounding_box: Option<GeoBBox2D>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NestedPlantingRun {
    pub planting_run: PlantingRunPersistent,
    pub plants: Vec<PlantPersistent>,
}

impl<Conn> DbEntry<Conn> for NestedPlantingRun {
    type B = Relational;
    type ID = i64;
}

#[gen_read_tests(id = self.planting_run.id, testable_ids = 1..4)]
impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for NestedPlantingRun {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        let entry = match dsl::planting_run.find(id).first::<PlantingRunPersistent>(c) {
            Ok(e) => e,
            Err(e) => match e {
                diesel::result::Error::NotFound => {
                    return Ok(None);
                }
                _ => return Err(DatabaseError::OperationError(e)),
            },
        };
        let plants = PlantPersistent::belonging_to(&entry)
            .load::<PlantPersistent>(c)
            .map_err(DatabaseError::from)?;

        Ok(Some(NestedPlantingRun {
            planting_run: entry,
            plants,
        }))
    }
}

#[gen_batch_read_tests(limit = 3)]
impl<Conn: LoadConnection<Backend = Pg>> Searchable<Conn> for NestedPlantingRun {
    type Filter<'a> = (Option<i64>, Option<GeoBBox2D>);

    fn search(
        c: &mut Conn,
        (field_id, box2d): Self::Filter<'_>,
        offset: usize,
        count: usize,
    ) -> Result<Vec<Self>, DatabaseError> {
        use postgis_diesel::operators::intersects_2d;
        let mut query = dsl::planting_run
            .offset(offset as i64)
            .limit(count as i64)
            .into_boxed();
        if let Some(bbox) = box2d {
            query = query.filter(intersects_2d(dsl::bounding_box, bbox));
        }
        if let Some(field_id) = field_id {
            query = query.filter(dsl::field_id.eq(field_id));
        }
        let entries = query
            .load::<PlantingRunPersistent>(c)
            .map_err(DatabaseError::from)?;

        let plants = PlantPersistent::belonging_to(&entries)
            .load::<PlantPersistent>(c)
            .map_err(DatabaseError::from)?
            .grouped_by(&entries);

        let data = entries
            .into_iter()
            .zip(plants)
            .map(|(e, p)| NestedPlantingRun {
                planting_run: e,
                plants: p,
            })
            .collect::<Vec<_>>();

        Ok(data)
    }
}

#[gen_delete_tests]
impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for NestedPlantingRun {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        diesel::delete(dsl::planting_run.filter(dsl::id.eq(id)))
            .returning(dsl::id)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

//////////////// Inserts ////////////////
#[derive(Insertable, Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(table_name = planting_run)]
pub struct PlantingRunInsert {
    pub user_id: i64,
    pub terminal_id: i64,
    #[cfg_attr(test, mockable_const(3))]
    pub field_id: i64,
    pub bounding_box: Option<GeoBBox2D>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(Mockable, LabelledGeneric))]
pub struct NestedPlantingRunInsert {
    pub planting_run_insert: PlantingRunInsert,
    pub plants_insert: Vec<PlantInsert>,
}

#[gen_create_tests]
impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, NestedPlantingRun>
    for NestedPlantingRunInsert
{
    fn create(mut self, c: &mut Conn) -> Result<i64, DatabaseError> {
        use crate::schema::plant::dsl::plant;

        c.transaction(|conn| {
            //Insert top level entry
            let planting_run_result = diesel::insert_into(dsl::planting_run)
                .values(&self.planting_run_insert)
                .get_result::<PlantingRunPersistent>(conn)
                .map_err(DatabaseError::from)?;
            for p in &mut self.plants_insert {
                p.planting_run_id = planting_run_result.id;
            }
            diesel::insert_into(plant)
                .values(&self.plants_insert)
                .execute(conn)
                .map_err(DatabaseError::from)?;
            Ok(planting_run_result.id)
        })
    }
}
