use crate::{
    diesel::prelude::*,
    schema::{forecast::dsl, *},
    DatabaseError, Relational,
};
use diesel::{connection::LoadConnection, pg::Pg};
use diesel_ext::db_backend::{BatchReadable, Creatable, DbEntry, Deletable, Readable};

//////////////// Queries ////////////////
#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = forecast)]
pub struct ForecastPersistent {
    pub id: i64,
    pub group_id: i64,
    pub station_id: i64,
    pub station: String,
    pub field_ids: Vec<i64>,
    pub model_id: i64,
    pub model: String,
    pub query_timestamp: chrono::NaiveDateTime,
    pub time_span_start: chrono::NaiveDate,
    pub time_span_end: chrono::NaiveDate,
    pub measurement_rate: String,
}

#[derive(Debug)]
pub struct NestedForecast {
    pub forecast: ForecastPersistent,
    pub measurements: Vec<ForecastMeasurementPersistent>,
}

impl<Conn> DbEntry<Conn> for NestedForecast {
    type B = Relational;
    type ID = i64;
}

impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for NestedForecast {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        diesel::delete(dsl::forecast.filter(dsl::id.eq(id)))
            .returning(dsl::id)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for NestedForecast {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        let entry = match dsl::forecast.find(id).first::<ForecastPersistent>(c) {
            Ok(e) => e,
            Err(e) => match e {
                diesel::result::Error::NotFound => {
                    return Ok(None);
                }
                _ => return Err(DatabaseError::OperationError(e)),
            },
        };
        let measurements = ForecastMeasurementPersistent::belonging_to(&entry)
            .load::<ForecastMeasurementPersistent>(c)
            .map_err(DatabaseError::from)?;
        let data = NestedForecast {
            forecast: entry,
            measurements: measurements.into_iter().collect(),
        };
        Ok(Some(data))
    }
}

impl<Conn: LoadConnection<Backend = Pg>> BatchReadable<Conn> for NestedForecast {
    fn read_all(c: &mut Conn, offset: usize, count: usize) -> Result<Vec<Self>, DatabaseError> {
        let entries = dsl::forecast
            .offset(offset as i64)
            .limit(count as i64)
            .load::<ForecastPersistent>(c)
            .map_err(DatabaseError::from)?;

        let measurements = ForecastMeasurementPersistent::belonging_to(&entries)
            .load::<ForecastMeasurementPersistent>(c)
            .map_err(DatabaseError::from)?
            .grouped_by(&entries);

        let data = entries
            .into_iter()
            .zip(measurements)
            .map(|(e, p)| NestedForecast {
                forecast: e,
                measurements: p,
            })
            .collect::<Vec<_>>();
        Ok(data)
    }
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[diesel(belongs_to(ForecastPersistent, foreign_key = forecast_id))]
#[diesel(table_name = forecast_measurement)]
pub struct ForecastMeasurementPersistent {
    pub id: i64,
    pub forecast_id: i64,
    pub time: chrono::NaiveDateTime,
    pub value: serde_json::Value,
}

//////////////// Inserts ////////////////
#[derive(Insertable, Debug)]
#[diesel(table_name = forecast)]
pub struct ForecastInsert {
    pub group_id: i64,
    pub station_id: i64,
    pub station: String,
    pub field_ids: Vec<i64>,
    pub model_id: i64,
    pub model: String,
    pub query_timestamp: chrono::NaiveDateTime,
    pub time_span_start: chrono::NaiveDate,
    pub time_span_end: chrono::NaiveDate,
    pub measurement_rate: String,
}

#[derive(Insertable)]
#[diesel(table_name = forecast_measurement)]
pub struct ForecastMeasurementInsert {
    pub forecast_id: i64,
    pub time: chrono::NaiveDateTime,
    pub value: serde_json::Value,
}

#[derive(Debug)]
pub struct NestedForecastInsert {
    forecast_insert: ForecastInsert,
    measurements_insert: Vec<(chrono::NaiveDateTime, serde_json::Value)>,
}

impl NestedForecastInsert {
    pub fn new(
        forecast_insert: ForecastInsert,
        measurements_insert: Vec<(chrono::NaiveDateTime, serde_json::Value)>,
    ) -> Self {
        Self {
            forecast_insert,
            measurements_insert,
        }
    }
}

impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, NestedForecast> for NestedForecastInsert {
    fn create(self, c: &mut Conn) -> Result<i64, DatabaseError> {
        use crate::schema::forecast_measurement::dsl as m_dsl;
        //Start Transaction
        c.transaction::<_, DatabaseError, _>(|conn| {
            //Insert top level entry
            let forecast_id = diesel::insert_into(dsl::forecast)
                .values(self.forecast_insert)
                .get_result::<ForecastPersistent>(conn)?
                .id;
            if !self.measurements_insert.is_empty() {
                let inserts: Vec<_> = self
                    .measurements_insert
                    .into_iter()
                    .map(|(time, value)| ForecastMeasurementInsert {
                        forecast_id,
                        time,
                        value,
                    })
                    .collect();
                diesel::insert_into(m_dsl::forecast_measurement)
                    .values(&inserts)
                    .get_result::<ForecastMeasurementPersistent>(conn)?;
            }
            Ok(forecast_id)
        })
    }
}
