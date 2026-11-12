use crate::{
    diesel::prelude::*,
    field::planting_run::db::PlantingRunPersistent,
    schema::{plant::dsl, *},
    DatabaseError, Relational,
};
use diesel::{connection::LoadConnection, pg::Pg};
use diesel_ext::{
    db_backend::{Creatable, DbEntry, Deletable, Readable, Searchable, Updatable},
    GeoBBox2D,
};
#[cfg(test)]
use macros_lib::Mockable;
use macros_lib::{
    gen_batch_read_tests, gen_create_tests, gen_delete_tests, gen_read_tests, gen_update_tests,
};
use postgis_diesel::gps::*;

//////////////// Queries ////////////////
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone, LabelledGeneric)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(belongs_to(PlantingRunPersistent, foreign_key = planting_run_id))]
#[diesel(table_name = plant)]
pub struct PlantPersistent {
    pub id: i64,
    #[cfg_attr(test, mockable_const(3))]
    pub planting_run_id: i64,
    pub import_key: String,
    pub timestamp: chrono::NaiveDateTime,
    pub height: f64,
    pub deviation: f64,
    pub speed: f64,
    pub plant_distance: f64,
    pub row_distance: f64,
    pub row_number: i32,
    pub geo_point: Point,
}

impl<Conn> DbEntry<Conn> for PlantPersistent {
    type B = Relational;
    type ID = i64;
}

#[gen_read_tests(testable_ids = 1..1543)]
impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for PlantPersistent {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        let entry = match dsl::plant.find(id).first::<PlantPersistent>(c) {
            Ok(e) => e,
            Err(e) => match e {
                diesel::result::Error::NotFound => {
                    return Ok(None);
                }
                _ => return Err(DatabaseError::OperationError(e)),
            },
        };
        Ok(Some(entry))
    }
}

#[gen_batch_read_tests(limit = 1542)]
impl<Conn: LoadConnection<Backend = Pg>> Searchable<Conn> for PlantPersistent {
    type Filter<'a> = (Option<i64>, Option<GeoBBox2D>);

    fn search(
        c: &mut Conn,
        (planter_id, box2d): Self::Filter<'_>,
        offset: usize,
        count: usize,
    ) -> Result<Vec<Self>, DatabaseError> {
        use postgis_diesel::operators::intersects_2d;
        let mut query = dsl::plant
            .offset(offset as i64)
            .limit(count as i64)
            .into_boxed();
        if let Some(planter_id) = planter_id {
            query = query.filter(dsl::planting_run_id.eq(planter_id));
        }
        if let Some(bbox) = box2d {
            query = query.filter(intersects_2d(dsl::geo_point, bbox));
        }
        let entries = query
            .load::<PlantPersistent>(c)
            .map_err(DatabaseError::from)?;

        Ok(entries)
    }
}

#[gen_delete_tests]
impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for PlantPersistent {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        diesel::delete(dsl::plant.filter(dsl::id.eq(id)))
            .returning(dsl::id)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

#[derive(AsChangeset, Debug, Clone, LabelledGeneric, Default, PartialEq)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(table_name = plant)]
pub struct PlantChangeSet {
    pub import_key: Option<String>,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub height: Option<f64>,
    pub deviation: Option<f64>,
    pub speed: Option<f64>,
    pub plant_distance: Option<f64>,
    pub row_distance: Option<f64>,
    pub row_number: Option<i32>,
}

#[gen_update_tests]
impl<Conn: LoadConnection<Backend = Pg>> Updatable<Conn, PlantPersistent> for PlantChangeSet {
    fn update(self, c: &mut Conn, id: &i64) -> Result<Option<PlantPersistent>, DatabaseError> {
        diesel::update(dsl::plant.filter(dsl::id.eq(id)))
            .set(&self)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

#[derive(Insertable, Debug, Clone, LabelledGeneric, PartialEq)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(table_name = plant)]
pub struct PlantInsert {
    #[cfg_attr(test, mockable_const(3))]
    pub planting_run_id: i64,
    pub import_key: String,
    pub timestamp: chrono::NaiveDateTime,
    pub height: f64,
    pub deviation: f64,
    pub speed: f64,
    pub plant_distance: f64,
    pub row_distance: f64,
    pub row_number: i32,
    pub geo_point: Point,
}

#[gen_create_tests]
impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, PlantPersistent> for PlantInsert {
    fn create(self, c: &mut Conn) -> Result<i64, DatabaseError> {
        c.transaction::<_, DatabaseError, _>(|conn| {
            Ok(diesel::insert_into(dsl::plant)
                .values(&self)
                .get_result::<PlantPersistent>(conn)?
                .id)
        })
    }
}
