use super::{
    schema::{jkilidardata::dsl, *},
    DatabaseError,
};
use crate::{diesel::prelude::*, JkiLidar};
use diesel::{connection::LoadConnection, pg::Pg};
use diesel_ext::db_backend::{BatchReadable, Creatable, DbEntry, Deletable, Readable};

use serde::{Deserialize, Serialize};

//////////////// Queries ////////////////
#[derive(Identifiable, Queryable, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[diesel(primary_key(jki_lidar_id))]
#[diesel(table_name = jkilidardata)]
pub struct Jkilidardata {
    pub jki_lidar_id: i32,
    pub longitude: f64,
    pub latitude: f64,
    pub altitude: f64,
    pub gpstime: Option<f64>,
    pub intensity: Option<i32>,
    pub return_number: Option<i32>,
    pub number_of_returns: Option<i32>,
    pub scan_direction_flag: Option<i32>,
    pub edge_of_flight_line: Option<i32>,
    pub classification: Option<i32>,
    pub synthetic: Option<bool>,
    pub keypoint: Option<bool>,
    pub withheld: Option<bool>,
    pub scan_angle_rank: Option<i32>,
    pub user_data: Option<i32>,
    pub point_source_id: Option<i32>,
}

impl<Conn> DbEntry<Conn> for Jkilidardata {
    type B = JkiLidar;
    type ID = i32;
}

impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for Jkilidardata {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        dsl::jkilidardata
            .find(id)
            .first(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

impl<Conn: LoadConnection<Backend = Pg>> BatchReadable<Conn> for Jkilidardata {
    fn read_all(c: &mut Conn, offset: usize, count: usize) -> Result<Vec<Self>, DatabaseError> {
        dsl::jkilidardata
            .offset(offset as i64)
            .limit(count as i64)
            .get_results(c)
            .map_err(DatabaseError::from)
    }
}

impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for Jkilidardata {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        diesel::delete(dsl::jkilidardata.find(id))
            .returning(dsl::jki_lidar_id)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

//////////////// Inserts ////////////////
#[derive(Insertable)]
#[diesel(table_name = jkilidardata)]
pub struct JkilidarInsert {
    pub longitude: f64,
    pub latitude: f64,
    pub altitude: f64,
    pub gpstime: Option<f64>,
    pub intensity: Option<i32>,
    pub return_number: Option<i32>,
    pub number_of_returns: Option<i32>,
    pub scan_direction_flag: Option<i32>,
    pub edge_of_flight_line: Option<i32>,
    pub classification: Option<i32>,
    pub synthetic: Option<bool>,
    pub keypoint: Option<bool>,
    pub withheld: Option<bool>,
    pub scan_angle_rank: Option<i32>,
    pub user_data: Option<i32>,
    pub point_source_id: Option<i32>,
}

impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, Jkilidardata> for JkilidarInsert {
    fn create(self, c: &mut Conn) -> Result<i32, DatabaseError> {
        diesel::insert_into(dsl::jkilidardata)
            .values(&self)
            .returning(dsl::jki_lidar_id)
            .get_result(c)
            .map_err(DatabaseError::from)
    }
}
