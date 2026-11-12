use crate::{
    diesel::prelude::*,
    schema::{quality_measurement::dsl, *},
    DatabaseError, Relational,
};
use diesel::{connection::LoadConnection, pg::Pg};
use diesel_ext::db_backend::{BatchReadable, Creatable, DbEntry, Deletable, Readable, Searchable};

use postgis_diesel::gps::*;
//////////////// Queries ////////////////
#[derive(Identifiable, Insertable, Queryable, PartialEq, Debug)]
#[diesel(table_name = quality_measurement)]
pub struct QualityMeasurementPersistent {
    pub id: i64,
    pub plant_id: i64,
    pub timestamp: chrono::NaiveDateTime,
    pub device: String,
    pub measurement_uuid: String,
    pub tartaric_acid: f64,
    pub malic_acid: f64,
    pub fructose: f64,
    pub glucose: f64,
    pub geo_point: Point,
}

impl<Conn> DbEntry<Conn> for QualityMeasurementPersistent {
    type B = Relational;
    type ID = i64;
}

impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for QualityMeasurementPersistent {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        let entry = match dsl::quality_measurement
            .find(id)
            .first::<QualityMeasurementPersistent>(c)
        {
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

impl<Conn: LoadConnection<Backend = Pg>> Searchable<Conn> for QualityMeasurementPersistent {
    /// plant id
    type Filter<'a> = Option<i64>;

    fn search(
        c: &mut Conn,
        filter: Self::Filter<'_>,
        offset: usize,
        count: usize,
    ) -> Result<Vec<Self>, DatabaseError> {
        let entries = match filter {
            Some(plant_id) => dsl::quality_measurement
                .filter(dsl::plant_id.eq(plant_id))
                .offset(offset as i64)
                .limit(count as i64)
                .load(c)
                .map_err(DatabaseError::from)?,
            None => dsl::quality_measurement
                .offset(offset as i64)
                .limit(count as i64)
                .load(c)
                .map_err(DatabaseError::from)?,
        };
        Ok(entries)
    }
}

impl<Conn: LoadConnection<Backend = Pg>> BatchReadable<Conn> for QualityMeasurementPersistent {
    fn read_all(c: &mut Conn, offset: usize, count: usize) -> Result<Vec<Self>, DatabaseError> {
        Self::search(c, None, offset, count)
    }
}

impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for QualityMeasurementPersistent {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        diesel::delete(dsl::quality_measurement.filter(dsl::id.eq(id)))
            .returning(dsl::id)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = quality_measurement)]
pub struct QualityMeasurementInsert {
    pub plant_id: i64,
    pub timestamp: chrono::NaiveDateTime,
    pub device: String,
    pub measurement_uuid: String,
    pub tartaric_acid: f64,
    pub malic_acid: f64,
    pub fructose: f64,
    pub glucose: f64,
    pub geo_point: Point,
}

impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, QualityMeasurementPersistent>
    for QualityMeasurementInsert
{
    fn create(self, c: &mut Conn) -> Result<i64, DatabaseError> {
        c.transaction::<_, DatabaseError, _>(|conn| {
            Ok(diesel::insert_into(dsl::quality_measurement)
                .values(&self)
                .get_result::<QualityMeasurementPersistent>(conn)?
                .id)
        })
    }
}
