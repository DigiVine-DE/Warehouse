use crate::{
    diesel::prelude::*,
    schema::{weather_data::dsl, *},
    DatabaseError, Relational,
};
use diesel::{connection::LoadConnection, pg::Pg};
use diesel_ext::db_backend::{Creatable, DbEntry, Deletable, Readable, Searchable, Updatable};
#[cfg(test)]
use macros_lib::Mockable;
use macros_lib::{
    gen_batch_read_tests, gen_create_tests, gen_delete_tests, gen_read_tests, gen_update_tests,
};

#[derive(Identifiable, Queryable, Insertable, PartialEq, Debug, Clone)]
#[diesel(primary_key(station_id, measurement_date))]
#[diesel(table_name = weather_data)]
#[cfg_attr(test, derive(Mockable))]
pub struct WeatherData {
    #[cfg_attr(test, mockable_const(377))]
    pub station_id: i32,
    pub measurement_date: chrono::NaiveDate,
    pub fx: Option<f64>,
    pub fx_qn: Option<i16>,
    pub fm: Option<f64>,
    pub fm_qn: Option<i16>,
    pub rsk: Option<f64>,
    pub rsk_qn: Option<i16>,
    pub rskf: Option<i32>,
    pub sdk: Option<f64>,
    pub sdk_qn: Option<i16>,
    pub shk_tag: Option<f64>,
    pub shk_tag_qn: Option<i16>,
    pub nm: Option<f64>,
    pub nm_qn: Option<i16>,
    pub vpm: Option<f64>,
    pub vpm_qn: Option<i16>,
    pub pm: Option<f64>,
    pub pm_qn: Option<i16>,
    pub tmk: Option<f64>,
    pub tmk_qn: Option<i16>,
    pub upm: Option<f64>,
    pub upm_qn: Option<i16>,
    pub txk: Option<f64>,
    pub txk_qn: Option<i16>,
    pub tnk: Option<f64>,
    pub tnk_qn: Option<i16>,
    pub tgk: Option<f64>,
    pub tgk_qn: Option<i16>,
}

impl<Conn> DbEntry<Conn> for WeatherData {
    type B = Relational;
    type ID = (i32, chrono::NaiveDate);
}

// The Readable test for WeatherData are written manually below
impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for WeatherData {
    fn read_one(
        c: &mut Conn,
        (station_id, measurement_date): Self::ID,
    ) -> Result<Option<Self>, DatabaseError> {
        dsl::weather_data
            .filter(dsl::station_id.eq(station_id))
            .filter(dsl::measurement_date.eq(measurement_date))
            .first(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

#[gen_delete_tests]
impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for WeatherData {
    fn delete(
        c: &mut Conn,
        (station_id, measurement_date): Self::ID,
    ) -> Result<Option<Self::ID>, DatabaseError> {
        diesel::delete(
            dsl::weather_data
                .filter(dsl::station_id.eq(station_id))
                .filter(dsl::measurement_date.eq(measurement_date)),
        )
        .returning((dsl::station_id, dsl::measurement_date))
        .get_result(c)
        .optional()
        .map_err(DatabaseError::from)
    }
}

#[derive(Default, Debug, Clone)]
pub struct WeatherDataFilter<'a> {
    pub station_ids: Option<&'a Vec<i32>>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
}

#[gen_batch_read_tests(limit = 100)]
impl<Conn: LoadConnection<Backend = Pg>> Searchable<Conn> for WeatherData {
    type Filter<'a> = WeatherDataFilter<'a>;

    fn search(
        c: &mut Conn,
        filter: Self::Filter<'_>,
        offset: usize,
        count: usize,
    ) -> Result<Vec<Self>, DatabaseError> {
        let mut query = dsl::weather_data.into_boxed();
        if let Some(ids) = filter.station_ids {
            query = query.filter(dsl::station_id.eq_any(ids));
        }
        if let Some(start) = filter.start_date {
            query = query.filter(dsl::measurement_date.ge(start));
        }
        if let Some(end) = filter.end_date {
            query = query.filter(dsl::measurement_date.le(end));
        }
        query
            .offset(offset as i64)
            .limit(count as i64)
            .order(dsl::measurement_date.desc())
            .load(c)
            .map_err(DatabaseError::from)
    }
}

#[derive(AsChangeset, Debug, Clone, LabelledGeneric, Default, PartialEq)]
#[diesel(table_name = weather_data)]
#[cfg_attr(test, derive(Mockable))]
pub struct WeatherDataChangeSet {
    pub fx: Option<f64>,
    pub fx_qn: Option<i16>,
    pub fm: Option<f64>,
    pub fm_qn: Option<i16>,
    pub rsk: Option<f64>,
    pub rsk_qn: Option<i16>,
    pub rskf: Option<i32>,
    pub sdk: Option<f64>,
    pub sdk_qn: Option<i16>,
    pub shk_tag: Option<f64>,
    pub shk_tag_qn: Option<i16>,
    pub nm: Option<f64>,
    pub nm_qn: Option<i16>,
    pub vpm: Option<f64>,
    pub vpm_qn: Option<i16>,
    pub pm: Option<f64>,
    pub pm_qn: Option<i16>,
    pub tmk: Option<f64>,
    pub tmk_qn: Option<i16>,
    pub upm: Option<f64>,
    pub upm_qn: Option<i16>,
    pub txk: Option<f64>,
    pub txk_qn: Option<i16>,
    pub tnk: Option<f64>,
    pub tnk_qn: Option<i16>,
    pub tgk: Option<f64>,
    pub tgk_qn: Option<i16>,
}

#[gen_update_tests]
impl<Conn: LoadConnection<Backend = Pg>> Updatable<Conn, WeatherData> for WeatherDataChangeSet {
    fn update(
        self,
        c: &mut Conn,
        (station_id, measurement_date): &(i32, chrono::NaiveDate),
    ) -> Result<Option<WeatherData>, DatabaseError> {
        diesel::update(
            dsl::weather_data
                .filter(dsl::station_id.eq(station_id))
                .filter(dsl::measurement_date.eq(measurement_date)),
        )
        .set(&self)
        .get_result(c)
        .optional()
        .map_err(DatabaseError::from)
    }
}

#[gen_create_tests]
impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, WeatherData> for WeatherData {
    fn create(self, c: &mut Conn) -> Result<(i32, chrono::NaiveDate), DatabaseError> {
        let res = diesel::insert_into(dsl::weather_data)
            .values(self)
            .get_result::<Self>(c)?;
        Ok((res.station_id, res.measurement_date))
    }
}

#[cfg(test)]
pub mod weatherdata_read_tests {
    use super::WeatherData;
    use crate::weather::WeatherStation;
    use chrono::{Days, NaiveDate};
    use diesel_ext::db_tester::ReadTestable;
    use serial_test::serial;
    use std::iter::successors;

    impl ReadTestable for WeatherData {
        const NAME: &'static str = "weatherdata";

        fn testable_ids() -> Vec<Self::ID> {
            let start = NaiveDate::from_ymd_opt(1947, 1, 1).unwrap();
            let end = NaiveDate::from_ymd_opt(2023, 12, 30).unwrap();
            successors(Some(start), |d| d.checked_add_days(Days::new(1)))
                .take_while(|d| d == &end)
                .map(|d| {
                    WeatherStation::testable_ids()
                        .into_iter()
                        .map(move |id| (id, d))
                })
                .flatten()
                .collect()
        }

        fn missing_id() -> Self::ID {
            (
                WeatherStation::missing_id(),
                NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(),
            )
        }

        fn id(&self) -> Self::ID {
            (self.station_id, self.measurement_date)
        }
    }

    #[test]
    #[serial]
    fn weatherdata_read_test() {
        WeatherData::read_test()
    }
}
