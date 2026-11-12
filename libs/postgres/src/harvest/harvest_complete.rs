use crate::{
    diesel::prelude::*,
    schema::{harvestcomplete::dsl, *},
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

//////////////// Queries ////////////////
#[derive(PartialEq, Debug, Clone, Identifiable, Queryable, Insertable, AsChangeset)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(primary_key(task_id))]
#[diesel(table_name = harvestcomplete)]
pub struct HarvestCompletePersistent {
    #[cfg_attr(test, mockable_const(19))]
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
    pub geom: Option<Polygon>,
    pub average_tartaric_acid: Option<i64>,
    pub average_malic_acid: Option<i64>,
    pub average_glucose: Option<i64>,
    pub average_fructose: Option<i64>,
}

#[derive(PartialEq, Debug, Clone, Identifiable, Queryable, Associations, AsChangeset)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(belongs_to(HarvestCompletePersistent, foreign_key = task_id))]
#[diesel(table_name = harvestmeasurement)]
pub struct HarvestMeasurementPersistent {
    pub id: i64,
    pub task_id: i64,
    pub acid: i64,
    pub sugar: i64,
    pub spectral: Option<String>,
    pub geom: Option<Point>,
}

#[derive(Insertable, Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(table_name = harvestmeasurement)]
pub struct HarvestMeasurementInsert {
    #[cfg_attr(test, mockable_const(19))]
    pub task_id: i64,
    pub acid: i64,
    pub sugar: i64,
    pub spectral: Option<String>,
    pub geom: Option<Point>,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Mockable))]
pub struct NestedHarvestComplete {
    pub harvest: HarvestCompletePersistent,
    pub measurements: Vec<HarvestMeasurementPersistent>,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(test, derive(Mockable))]
pub struct NestedHarvestCompleteInsert {
    pub harvest: HarvestCompletePersistent,
    pub measurements: Vec<HarvestMeasurementInsert>,
}

impl<Conn> DbEntry<Conn> for NestedHarvestComplete {
    type B = Relational;
    type ID = i64;
}

#[gen_read_tests(id = self.harvest.task_id, testable_ids = 1..19)]
impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for NestedHarvestComplete {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        let entry = match dsl::harvestcomplete
            .find(id)
            .first::<HarvestCompletePersistent>(c)
        {
            Ok(e) => e,
            Err(e) => match e {
                diesel::result::Error::NotFound => {
                    return Ok(None);
                }
                _ => return Err(DatabaseError::OperationError(e)),
            },
        };
        let measurements = HarvestMeasurementPersistent::belonging_to(&entry)
            .load::<HarvestMeasurementPersistent>(c)
            .map_err(DatabaseError::from)?;

        Ok(Some(NestedHarvestComplete {
            harvest: entry,
            measurements,
        }))
    }
}

#[gen_batch_read_tests(limit = 18)]
impl<Conn: LoadConnection<Backend = Pg>> Searchable<Conn> for NestedHarvestComplete {
    type Filter<'a> = Option<GeoBBox2D>;

    fn search(
        c: &mut Conn,
        box2d: Self::Filter<'_>,
        offset: usize,
        count: usize,
    ) -> Result<Vec<Self>, DatabaseError> {
        use postgis_diesel::operators::intersects_2d;
        let mut query = dsl::harvestcomplete
            .offset(offset as i64)
            .limit(count as i64)
            .into_boxed();
        if let Some(bbox) = box2d {
            query = query.filter(intersects_2d(dsl::geom, bbox));
        }
        let entries = query
            .load::<HarvestCompletePersistent>(c)
            .map_err(DatabaseError::from)?;
        let measurements = HarvestMeasurementPersistent::belonging_to(&entries)
            .load::<HarvestMeasurementPersistent>(c)
            .map_err(DatabaseError::from)?
            .grouped_by(&entries);

        let result = entries.into_iter().zip(measurements);

        Ok(result
            .map(|(h, nm)| NestedHarvestComplete {
                harvest: h,
                measurements: nm,
            })
            .collect())
    }
}

#[gen_delete_tests]
impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for NestedHarvestComplete {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        diesel::delete(dsl::harvestcomplete.filter(dsl::task_id.eq(id)))
            .returning(dsl::task_id)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

#[gen_create_tests]
impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, NestedHarvestComplete>
    for NestedHarvestCompleteInsert
{
    fn create(mut self, c: &mut Conn) -> Result<i64, DatabaseError> {
        use crate::schema::harvesttask::dsl as task_dsl;

        let id = self.harvest.task_id;
        for m in &mut self.measurements {
            m.task_id = self.harvest.task_id;
        }

        c.transaction::<_, DatabaseError, _>(|conn| {
            //Insert top level entry
            diesel::insert_into(dsl::harvestcomplete)
                .values(&self.harvest)
                .execute(conn)?;

            // Insert measurements
            diesel::insert_into(harvestmeasurement::dsl::harvestmeasurement)
                .values(self.measurements)
                .execute(conn)?;

            // Remove task from running task table
            diesel::delete(task_dsl::harvesttask.filter(task_dsl::task_id.eq(self.harvest.task_id)))
                .execute(conn)
                .map_err(DatabaseError::from)
        })?;

        Ok(id)
    }
}
