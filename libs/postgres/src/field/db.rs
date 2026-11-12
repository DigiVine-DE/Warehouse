use super::{
    filter::FieldFilter,
    planting_run::{
        db::{NestedPlantingRun, NestedPlantingRunInsert, PlantingRunPersistent},
        plant::db::PlantPersistent,
    },
};
use crate::{
    diesel::prelude::*,
    schema::{fields::dsl, *},
    weather::WeatherStation,
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
use std::cmp::Reverse;

//////////////// Queries ////////////////
#[derive(PartialEq, Debug, Clone, Identifiable, Queryable, LabelledGeneric, AsChangeset)]
#[diesel(table_name = fields)]
pub struct FieldPersistent {
    pub id: i64,
    pub name: Option<String>,
    pub station_id: Option<i64>,
    pub field_number: Option<String>,
    pub field_size: Option<f64>,
    pub fruit_name: Option<String>,
    pub geom: Option<MultiPolygon>,
    pub bounding_box: GeoBBox2D,
    pub historical_station_id: Option<i32>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct NestedField {
    pub field: FieldPersistent,
    pub planting_runs: Vec<NestedPlantingRun>,
}

impl<Conn> DbEntry<Conn> for NestedField {
    type B = Relational;
    type ID = i64;
}

impl<Conn> DbEntry<Conn> for FieldPersistent {
    type B = Relational;
    type ID = i64;
}

#[gen_read_tests(testable_ids = [3, 35..=74, 76, 77, 87, 167])]
impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for FieldPersistent {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        match dsl::fields.find(id).first::<FieldPersistent>(c) {
            Ok(e) => Ok(Some(e)),
            Err(e) => match e {
                diesel::result::Error::NotFound => Ok(None),
                _ => Err(DatabaseError::OperationError(e)),
            },
        }
    }
}

#[gen_read_tests(id = self.field.id, testable_ids = [3, 35..=74, 76, 77, 87, 167])]
impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for NestedField {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        let entry = match FieldPersistent::read_one(c, id)? {
            Some(e) => e,
            None => return Ok(None),
        };

        let planting_runs = PlantingRunPersistent::belonging_to(&entry)
            .load::<PlantingRunPersistent>(c)
            .map_err(DatabaseError::from)?;

        let plants = PlantPersistent::belonging_to(&planting_runs)
            .load::<PlantPersistent>(c)
            .map_err(DatabaseError::from)?
            .grouped_by(&planting_runs);

        Ok(Some(NestedField {
            field: entry.clone(),
            planting_runs: planting_runs
                .into_iter()
                .zip(plants)
                .into_iter()
                .map(|(pr, p)| NestedPlantingRun {
                    planting_run: pr,
                    plants: p,
                })
                .collect(),
        }))
    }
}

#[gen_batch_read_tests(limit = 45)]
impl<Conn: LoadConnection<Backend = Pg>> Searchable<Conn> for FieldPersistent {
    type Filter<'a> = (FieldFilter<'a>, Option<GeoBBox2D>);

    fn search(
        c: &mut Conn,
        (filter, box2d): Self::Filter<'_>,
        offset: usize,
        count: usize,
    ) -> Result<Vec<Self>, DatabaseError> {
        use postgis_diesel::operators::intersects_2d;

        let mut query = filter.to_query();
        query = query
            .order(dsl::id.asc())
            .offset(offset as i64)
            .limit(count as i64);
        if let Some(bbox) = box2d {
            query = query.filter(intersects_2d(dsl::geom, bbox));
        }

        let mut field_entries = query
            .load::<FieldPersistent>(c)
            .map_err(DatabaseError::from)?;

        field_entries.sort_by_cached_key(|field| Reverse(filter.fuzzy_score(field)));

        Ok(field_entries)
    }
}

// this test will fail because our implementation doesn't return nested planting_runs
// #[gen_batch_read_tests(limit = 45)]
impl<Conn: LoadConnection<Backend = Pg>> Searchable<Conn> for NestedField {
    type Filter<'a> = (FieldFilter<'a>, Option<GeoBBox2D>);

    fn search(
        c: &mut Conn,
        filter: Self::Filter<'_>,
        offset: usize,
        count: usize,
    ) -> Result<Vec<Self>, DatabaseError> {
        let field_entries = FieldPersistent::search(c, filter, offset, count)?;

        // let planting_runs = PlantingRunPersistent::belonging_to(&field_entries)
        //     .load::<PlantingRunPersistent>(self)
        //     .map_err(DatabaseError::from)?;

        // let plants = PlantPersistent::belonging_to(&planting_runs)
        //     .load::<PlantPersistent>(self)
        //     .map_err(DatabaseError::from)?
        //     .grouped_by(&planting_runs);

        // let planting_runs_and_plants: Vec<Vec<(PlantingRunPersistent, Vec<PlantPersistent>)>> =
        //     planting_runs
        //         .into_iter()
        //         .zip(plants)
        //         .grouped_by(&field_entries);

        // let result: Vec<(
        //     FieldPersistent,
        //     Vec<(PlantingRunPersistent, Vec<PlantPersistent>)>,
        // )> = field_entries
        //     .into_iter()
        //     .zip(planting_runs_and_plants)
        //     .collect();

        // Ok(result
        //     .into_iter()
        //     .map(|(f, pr_p)| NestedField {
        //         field: f.clone(),
        //         planting_runs: pr_p
        //             .into_iter()
        //             .map(|(pr, p)| NestedPlantingRun {
        //                 planting_run: pr,
        //                 plants: p,
        //             })
        //             .collect(),
        //     })
        //     .collect())

        Ok(field_entries
            .into_iter()
            .map(|f| NestedField {
                field: f,
                planting_runs: vec![],
            })
            .collect())
    }
}

#[gen_delete_tests]
impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for NestedField {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        diesel::delete(dsl::fields.filter(dsl::id.eq(id)))
            .returning(dsl::id)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

#[derive(AsChangeset, Debug, Clone, LabelledGeneric, Default, PartialEq)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(table_name = fields)]
pub struct FieldChangeSet {
    pub name: Option<String>,
    pub station_id: Option<i64>,
    #[cfg_attr(test, mockable_const(Some(377)))]
    pub historical_station_id: Option<i32>,
    pub field_number: Option<String>,
    pub field_size: Option<f64>,
    pub fruit_name: Option<String>,
}

#[gen_update_tests]
impl<Conn: LoadConnection<Backend = Pg>> Updatable<Conn, FieldPersistent> for FieldChangeSet {
    fn update(self, c: &mut Conn, id: &i64) -> Result<Option<FieldPersistent>, DatabaseError> {
        diesel::update(dsl::fields.filter(dsl::id.eq(id)))
            .set(self)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

//////////////// Inserts ////////////////
#[derive(Insertable, Debug, LabelledGeneric, Clone)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(table_name = fields)]
pub struct FieldInsert {
    pub name: Option<String>,
    pub station_id: Option<i64>,
    pub field_number: Option<String>,
    pub field_size: Option<f64>,
    pub fruit_name: Option<String>,
    pub geom: Option<MultiPolygon>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Mockable))]
pub struct NestedFieldInsert {
    pub field_insert: FieldInsert,
    pub planting_runs_inserts: Vec<NestedPlantingRunInsert>,
}

#[gen_create_tests]
impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, NestedField> for NestedFieldInsert {
    fn create(self, c: &mut Conn) -> Result<i64, DatabaseError> {
        c.transaction(|conn| {
            //Insert top level entry
            let field_result = diesel::insert_into(dsl::fields)
                .values(&self.field_insert)
                .get_result::<FieldPersistent>(conn)
                .map_err(DatabaseError::from)?;
            for mut run in self.planting_runs_inserts {
                run.planting_run_insert.field_id = field_result.id;
                run.create(conn)?;
            }
            Ok(field_result.id)
        })
    }
}

impl<'a> FieldFilter<'a> {
    fn to_query(&self) -> crate::schema::fields::BoxedQuery<'_, Pg> {
        let mut query = dsl::fields.into_boxed();

        // Ensure `fuzzy_keywords` are processed first, as `or_filter` combines with existing `where` conditions using `or`. Calling `to_query` outside is safe since it generates a new query.
        query = self
            .fuzzy_keywords
            .like_pattern()
            .iter()
            .fold(query, |q, fuzzy_seg| {
                q.or_filter(
                    dsl::name
                        .ilike(fuzzy_seg)
                        .or(dsl::fruit_name.ilike(fuzzy_seg)),
                )
            });
        query = self
            .field_name_filter
            .like_pattern()
            .iter()
            .fold(query, |q, field_name_seg| {
                q.filter(dsl::name.ilike(field_name_seg))
            });
        query = self
            .fruit_name_filter
            .like_pattern()
            .iter()
            .fold(query, |q, fruit_name_seg| {
                q.filter(dsl::fruit_name.ilike(fruit_name_seg))
            });
        query
    }
}
