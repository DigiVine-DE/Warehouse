use crate::{
    diesel::prelude::*,
    schema::{weather_station::dsl, *},
    DatabaseError, Relational,
};
use diesel::{connection::LoadConnection, pg::Pg};
use diesel_ext::{
    db_backend::{Creatable, DbEntry, Deletable, Readable, Searchable, Updatable},
    GeoBBox2D, Keywords,
};
#[cfg(test)]
use macros_lib::Mockable;
use macros_lib::{
    gen_batch_read_tests, gen_create_tests, gen_delete_tests, gen_read_tests, gen_update_tests,
};
use postgis_diesel::gps::PointZ;

#[derive(Identifiable, Queryable, PartialEq, Debug, Insertable, Clone)]
#[diesel(primary_key(station_id))]
#[diesel(table_name = weather_station)]
#[cfg_attr(test, derive(Mockable))]
pub struct WeatherStation {
    pub station_id: i32,
    pub station_name: Option<String>,
    pub station_bundesland: Option<String>,
    pub station_geo_point_z: Option<PointZ>,
}

impl<Conn> DbEntry<Conn> for WeatherStation {
    type B = Relational;
    type ID = i32;
}

#[gen_read_tests(id = self.station_id, testable_ids = [3, 164, 183, 377, 15000])]
impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for WeatherStation {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        dsl::weather_station
            .find(id)
            .first::<WeatherStation>(c)
            .optional()
            .map_err(|e| DatabaseError::OperationError(e))
    }
}

#[gen_batch_read_tests(limit = 5)]
impl<Conn: LoadConnection<Backend = Pg>> Searchable<Conn> for WeatherStation {
    type Filter<'a> = (WeatherStationFilter<'a>, Option<GeoBBox2D>);

    fn search(
        c: &mut Conn,
        (filter, box2d): Self::Filter<'_>,
        offset: usize,
        count: usize,
    ) -> Result<Vec<Self>, DatabaseError> {
        use postgis_diesel::operators::intersects_2d;

        let mut query = filter.to_query();
        query = query.offset(offset as i64).limit(count as i64);
        if let Some(bbox) = box2d {
            query = query.filter(intersects_2d(dsl::station_geo_point_z, bbox));
        }

        query.load(c).map_err(DatabaseError::from)
    }
}

#[gen_delete_tests]
impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for WeatherStation {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        diesel::delete(dsl::weather_station.filter(dsl::station_id.eq(id)))
            .returning(dsl::station_id)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

#[gen_create_tests]
impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, WeatherStation> for WeatherStation {
    fn create(self, c: &mut Conn) -> Result<i32, DatabaseError> {
        Ok(diesel::insert_into(dsl::weather_station)
            .values(&self)
            .get_result::<Self>(c)
            .map_err(DatabaseError::from)?
            .station_id)
    }
}

#[derive(AsChangeset, Debug, Clone, LabelledGeneric, Default, PartialEq)]
#[diesel(table_name = weather_station)]
#[cfg_attr(test, derive(Mockable))]
pub struct WeatherStationChangeSet {
    pub station_name: Option<String>,
    pub station_bundesland: Option<String>,
    pub station_geo_point_z: Option<PointZ>,
}

#[gen_update_tests]
impl<Conn: LoadConnection<Backend = Pg>> Updatable<Conn, WeatherStation>
    for WeatherStationChangeSet
{
    fn update(self, c: &mut Conn, id: &i32) -> Result<Option<WeatherStation>, DatabaseError> {
        diesel::update(dsl::weather_station.filter(dsl::station_id.eq(id)))
            .set(self)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

#[derive(Default, Debug, Clone)]
pub struct WeatherStationFilter<'a> {
    pub name_filter: Keywords<'a>,
    pub bundesland_filter: Keywords<'a>,
}

impl<'a> WeatherStationFilter<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_name_filter(mut self, words: impl Into<Keywords<'a>>) -> Self {
        self.name_filter = words.into();
        self
    }

    #[must_use]
    pub fn with_bundesland_filter(mut self, words: impl Into<Keywords<'a>>) -> Self {
        self.bundesland_filter = words.into();
        self
    }

    fn to_query(&self) -> crate::schema::weather_station::BoxedQuery<'_, Pg> {
        let mut query = dsl::weather_station.into_boxed();

        query = self
            .name_filter
            .like_pattern()
            .iter()
            .fold(query, |q, seg| q.filter(dsl::station_name.ilike(seg)));

        query = self
            .bundesland_filter
            .like_pattern()
            .iter()
            .fold(query, |q, seg| q.filter(dsl::station_bundesland.ilike(seg)));

        query
    }
}
