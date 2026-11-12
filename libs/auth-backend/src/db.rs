use super::schema;
use crate::{diesel::prelude::*, filter::TokenFilter, Auth, AuthLibError, DatabaseError};
use diesel::{connection::LoadConnection, dsl::sql, pg::Pg, sql_types};
use diesel_ext::db_backend::{
    BatchDeletable, Creatable, DbEntry, Deletable, Readable, Searchable, Upsertable,
};

#[derive(PartialEq, Debug, Clone, Identifiable, Queryable, AsChangeset)]
#[diesel(table_name = schema::token)]
pub struct TokenPersistent {
    pub id: i32,
    pub tokenstr: String,
    pub user_id: i32,
}

impl<Conn> DbEntry<Conn> for TokenPersistent {
    type B = Auth;
    type ID = i32;
}

impl<Conn: LoadConnection<Backend = Pg>> Searchable<Conn> for TokenPersistent {
    type Filter<'a> = TokenFilter<'a>;

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

pub fn has_access<Conn: LoadConnection<Backend = Pg>>(
    conn: &mut Conn,
    token: &str,
    path: &str,
    read: bool,
) -> Result<bool, AuthLibError> {
    let rel_path = path.replace(digivine_server::BASE_PATH, ""); //Only relative path
    let count = schema::token::table
        .inner_join(schema::accessrights::table)
        .filter(schema::token::tokenstr.eq(token))
        .filter(
            sql::<sql_types::Bool>("(path = '*' OR ")
                .bind::<sql_types::Text, _>(rel_path)
                .sql(" LIKE path)"),
        )
        .filter(
            schema::accessrights::readonly
                .eq(read)
                .or(schema::accessrights::readonly.eq(false)),
        )
        .count()
        .get_result::<i64>(conn)
        .map_err(DatabaseError::from)?;

    Ok(count > 0)
}

pub fn find_token_with_user<Conn: LoadConnection<Backend = Pg>>(
    conn: &mut Conn,
    user_id: i32,
    tokenstr: &str,
) -> Result<Option<TokenPersistent>, AuthLibError> {
    use schema::token::dsl;

    dsl::token
        .filter(dsl::tokenstr.eq(tokenstr))
        .filter(dsl::user_id.eq(user_id))
        .first(conn)
        .optional()
        .map_err(DatabaseError::from)
        .map_err(AuthLibError::from)
}

#[derive(PartialEq, Debug, Clone, Insertable)]
#[diesel(table_name = schema::token)]
pub struct TokenInsert {
    pub tokenstr: String,
    pub user_id: i32,
}

impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, TokenPersistent> for TokenInsert {
    fn create(self, c: &mut Conn) -> Result<i32, DatabaseError> {
        let tk: TokenPersistent = diesel::insert_into(schema::token::dsl::token)
            .values(self)
            .get_result(c)?;
        Ok(tk.id)
    }
}

impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for TokenPersistent {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        use schema::token::dsl;

        match diesel::delete(dsl::token.filter(dsl::id.eq(id)))
            .returning(dsl::id)
            .get_result(c)
        {
            Ok(id) => Ok(Some(id)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

impl<Conn: LoadConnection<Backend = Pg>> BatchDeletable<Conn> for TokenPersistent {
    type Filter<'a> = i32;

    fn batch_delete(c: &mut Conn, user_id: Self::Filter<'_>) -> Result<Vec<Self>, DatabaseError> {
        use schema::token::dsl;
        diesel::delete(dsl::token.filter(dsl::user_id.eq(user_id)))
            .get_results(c)
            .map_err(DatabaseError::from)
    }
}

#[derive(PartialEq, Debug, Clone, Identifiable, Queryable, AsChangeset, Insertable)]
#[diesel(table_name = schema::accessrights)]
#[diesel(primary_key(tid, path))]
pub struct AccessRightsPersistent {
    pub tid: i32,
    pub path: String,
    pub readonly: bool,
}

impl<Conn> DbEntry<Conn> for AccessRightsPersistent {
    type B = Auth;
    type ID = (i32, String);
}

impl<Conn: LoadConnection<Backend = Pg>> Searchable<Conn> for AccessRightsPersistent {
    type Filter<'a> = Option<i32>;

    fn search(
        c: &mut Conn,
        filter: Self::Filter<'_>,
        offset: usize,
        count: usize,
    ) -> Result<Vec<Self>, DatabaseError> {
        use schema::accessrights::dsl;

        if let Some(tid) = filter {
            dsl::accessrights
                .filter(dsl::tid.eq(tid))
                .offset(offset as i64)
                .limit(count as i64)
                .load(c)
                .map_err(DatabaseError::from)
        } else {
            dsl::accessrights
                .offset(offset as i64)
                .limit(count as i64)
                .load(c)
                .map_err(DatabaseError::from)
        }
    }
}

impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for AccessRightsPersistent {
    fn read_one(c: &mut Conn, (tid, path): Self::ID) -> Result<Option<Self>, DatabaseError> {
        use schema::accessrights::dsl;

        dsl::accessrights
            .filter(dsl::tid.eq(tid))
            .filter(dsl::path.eq(path))
            .first(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for AccessRightsPersistent {
    fn delete(c: &mut Conn, (tid, path): Self::ID) -> Result<Option<Self::ID>, DatabaseError> {
        use schema::accessrights::dsl;

        diesel::delete(dsl::accessrights.filter(dsl::tid.eq(tid).and(dsl::path.eq(path))))
            .returning((dsl::tid, dsl::path))
            .get_result(c)
            .optional()
            .map_err(DatabaseError::from)
    }
}

impl<Conn: LoadConnection<Backend = Pg>> Upsertable<Conn, Self> for AccessRightsPersistent {
    fn upsert(self, c: &mut Conn) -> Result<Self, DatabaseError> {
        use schema::accessrights::dsl;
        diesel::insert_into(dsl::accessrights)
            .values(&self)
            .on_conflict((dsl::tid, dsl::path))
            .do_update()
            .set(dsl::readonly.eq(self.readonly))
            .get_result(c)
            .map_err(DatabaseError::from)
    }
}
