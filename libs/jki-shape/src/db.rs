use super::schema::{jkishapedata::dsl, *};
use crate::{diesel::prelude::*, JkiShape};
use diesel::{connection::LoadConnection, pg::Pg};
use diesel_ext::db_backend::{
    BatchReadable, Creatable, DatabaseError, DbEntry, Deletable, Readable,
};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[diesel(primary_key(jki_shape_id))]
#[diesel(table_name = jkishapedata)]
pub struct JkiShapeData {
    pub jki_shape_id: i32,
    pub id: f64,
    pub gridcode: f64,
    pub orig_fid: f64,
}

impl<Conn> DbEntry<Conn> for JkiShapeData {
    type B = JkiShape;
    type ID = i32;
}

impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for JkiShapeData {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        match dsl::jkishapedata.find(id).first(c) {
            Ok(x) => Ok(Some(x)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

impl<Conn: LoadConnection<Backend = Pg>> BatchReadable<Conn> for JkiShapeData {
    fn read_all(c: &mut Conn, offset: usize, count: usize) -> Result<Vec<Self>, DatabaseError> {
        dsl::jkishapedata
            .offset(offset as i64)
            .limit(count as i64)
            .load(c)
            .map_err(DatabaseError::from)
    }
}

impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for JkiShapeData {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        diesel::delete(dsl::jkishapedata.find(id))
            .returning(dsl::jki_shape_id)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

//////////////// Inserts ////////////////
#[derive(Insertable)]
#[diesel(table_name = jkishapedata)]
pub struct JkiShapeInsert {
    pub id: f64,
    pub gridcode: f64,
    pub orig_fid: f64,
}

impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, JkiShapeData> for JkiShapeInsert {
    fn create(self, c: &mut Conn) -> Result<i32, DatabaseError> {
        diesel::insert_into(dsl::jkishapedata)
            .values(&self)
            .returning(dsl::jki_shape_id)
            .get_result(c)
            .map_err(DatabaseError::from)
    }
}
