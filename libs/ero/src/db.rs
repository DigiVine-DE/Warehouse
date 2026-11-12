use super::schema::{erodata::dsl, *};
use crate::{diesel::prelude::*, Ero};
use chrono::naive::serde::ts_seconds;
use diesel::{connection::LoadConnection, pg::Pg};
use diesel_ext::db_backend::{
    BatchReadable, Creatable, DatabaseError, DbEntry, Deletable, Readable,
};
use serde::{Deserialize, Serialize};

//////////////// Queries ////////////////

/// A GPS coordinate
#[derive(Identifiable, Queryable, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(primary_key(ero_id))]
#[diesel(table_name = erodata)]
pub struct Erodata {
    pub ero_id: i32,
    pub lat: f64,
    pub lon: f64,
    /// Corrected NDVI value
    pub ndvi_corrected: f64,
    #[serde(with = "ts_seconds")]
    pub time: chrono::NaiveDateTime,
    /// Raw NDVI value
    pub ndvi_raw: f64,
}

impl<Conn> DbEntry<Conn> for Erodata {
    type B = Ero;
    type ID = i32;
}

impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for Erodata {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        match dsl::erodata.find(id).first::<Erodata>(c) {
            Ok(x) => Ok(Some(x)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

impl<Conn: LoadConnection<Backend = Pg>> BatchReadable<Conn> for Erodata {
    fn read_all(c: &mut Conn, offset: usize, count: usize) -> Result<Vec<Self>, DatabaseError> {
        dsl::erodata
            .offset(offset as i64)
            .limit(count as i64)
            .load(c)
            .map_err(DatabaseError::from)
    }
}

impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for Erodata {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        diesel::delete(dsl::erodata.find(id))
            .returning(dsl::ero_id)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

//////////////// Inserts ////////////////
#[derive(Insertable)]
#[diesel(table_name = erodata)]
pub struct Eroinsert {
    pub lat: f64,
    pub lon: f64,
    pub ndvi_corrected: f64,
    pub time: chrono::NaiveDateTime,
    pub ndvi_raw: f64,
}

impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, Erodata> for Eroinsert {
    fn create(self, c: &mut Conn) -> Result<i32, DatabaseError> {
        diesel::insert_into(dsl::erodata)
            .values(&self)
            .returning(dsl::ero_id)
            .get_result(c)
            .map_err(DatabaseError::from)
    }
}
