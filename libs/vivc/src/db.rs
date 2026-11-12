use super::schema::{self, vivc::dsl};
use crate::Vivc;
use diesel::{connection::LoadConnection, pg::Pg, prelude::*};
use diesel_ext::{
    db_backend::{Creatable, DatabaseError, DbEntry, Deletable, Readable, Searchable, Updatable},
    Keywords,
};
#[cfg(test)]
use macros_lib::Mockable;
use macros_lib::{
    gen_batch_read_tests, gen_create_tests, gen_delete_tests, gen_read_tests, gen_update_tests,
};

#[derive(PartialEq, Debug, Clone, Identifiable, Queryable)]
#[diesel(table_name = schema::vivc)]
pub struct VivcPersistent {
    pub id: i32,
    pub cultivar_name: String,
    pub prime_name: String,
    pub variety_number: i32,
    pub species: Option<String>,
    pub color: Option<String>,
    pub country_or_region: Option<String>,
}

impl<Conn> DbEntry<Conn> for VivcPersistent {
    type B = Vivc;
    type ID = i32;
}

#[gen_read_tests(testable_ids = 1..79065)]
impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for VivcPersistent {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        dsl::vivc
            .find(id)
            .first(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

#[gen_delete_tests]
impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for VivcPersistent {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        diesel::delete(dsl::vivc.filter(dsl::id.eq(id)))
            .returning(dsl::id)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

#[derive(Insertable, Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(table_name = schema::vivc)]
pub struct VivcInsert {
    pub cultivar_name: String,
    pub prime_name: String,
    pub variety_number: i32,
    pub species: Option<String>,
    pub color: Option<String>,
    pub country_or_region: Option<String>,
}

#[gen_create_tests]
impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, VivcPersistent> for VivcInsert {
    fn create(self, c: &mut Conn) -> Result<i32, DatabaseError> {
        Ok(diesel::insert_into(dsl::vivc)
            .values(&self)
            .get_result::<VivcPersistent>(c)?
            .id)
    }
}

#[gen_batch_read_tests(limit = 100)]
impl<Conn: LoadConnection<Backend = Pg>> Searchable<Conn> for VivcPersistent {
    type Filter<'a> = CultivarFilter<'a>;

    fn search(
        c: &mut Conn,
        filter: Self::Filter<'_>,
        offset: usize,
        count: usize,
    ) -> Result<Vec<Self>, DatabaseError> {
        filter
            .to_query()
            .offset(offset as i64)
            .limit(count as i64)
            .load(c)
            .map_err(DatabaseError::from)
    }
}

#[derive(PartialEq, Debug, Clone, Queryable, AsChangeset)]
#[cfg_attr(test, derive(Mockable))]
#[diesel(table_name = schema::vivc)]
pub struct VivcChangeSet {
    pub cultivar_name: Option<String>,
    pub prime_name: Option<String>,
    pub variety_number: Option<i32>,
    pub species: Option<String>,
    pub color: Option<String>,
    pub country_or_region: Option<String>,
}

#[gen_update_tests]
impl<Conn: LoadConnection<Backend = Pg>> Updatable<Conn, VivcPersistent> for VivcChangeSet {
    fn update(self, c: &mut Conn, id: &i32) -> Result<Option<VivcPersistent>, DatabaseError> {
        diesel::update(dsl::vivc.filter(dsl::id.eq(id)))
            .set(self)
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

#[derive(Default, Debug, Clone)]
pub struct CultivarFilter<'a> {
    color_filter: Keywords<'a>,
    region_filter: Keywords<'a>,
    species_filter: Keywords<'a>,
    prime_name_filter: Keywords<'a>,
    cultivar_name_filter: Keywords<'a>,
}

impl<'a> CultivarFilter<'a> {
    pub fn to_query(&self) -> schema::vivc::BoxedQuery<'_, Pg> {
        let mut query = dsl::vivc.into_boxed();
        query = self
            .color_filter
            .like_pattern()
            .iter()
            .fold(query, |q, seg| q.filter(dsl::color.ilike(seg)));
        query = self
            .species_filter
            .like_pattern()
            .iter()
            .fold(query, |q, seg| q.filter(dsl::species.ilike(seg)));
        query = self
            .region_filter
            .like_pattern()
            .iter()
            .fold(query, |q, seg| q.filter(dsl::country_or_region.ilike(seg)));
        query = self
            .prime_name_filter
            .like_pattern()
            .iter()
            .fold(query, |q, seg| q.filter(dsl::prime_name.ilike(seg)));
        query = self
            .cultivar_name_filter
            .like_pattern()
            .iter()
            .fold(query, |q, seg| q.filter(dsl::cultivar_name.ilike(seg)));
        query
    }

    #[must_use]
    pub fn with_color(mut self, words: impl Into<Keywords<'a>>) -> Self {
        self.color_filter = words.into();
        self
    }

    #[must_use]
    pub fn with_region(mut self, words: impl Into<Keywords<'a>>) -> Self {
        self.region_filter = words.into();
        self
    }

    #[must_use]
    pub fn with_species(mut self, words: impl Into<Keywords<'a>>) -> Self {
        self.species_filter = words.into();
        self
    }

    #[must_use]
    pub fn with_prime_name(mut self, words: impl Into<Keywords<'a>>) -> Self {
        self.prime_name_filter = words.into();
        self
    }

    #[must_use]
    pub fn with_cultivar_name(mut self, words: impl Into<Keywords<'a>>) -> Self {
        self.cultivar_name_filter = words.into();
        self
    }
}
