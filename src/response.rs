use std::fmt::Debug;

use diesel_ext::db_backend::{
    Backend, BatchDeletable, BatchReadable, Creatable, DbEntry, Deletable, Readable, Searchable,
    Updatable, Upsertable,
};
use digivine_api_postgres::field::FieldFilter;
use digivine_server::{
    models, AsBackendErrorResp, AsBadRequestResponse, NotFoundResponse, UnitSuccessResponse,
};
use postgis_diesel::gps::PointZ;
use swagger::Nullable;

use crate::geo_conv::{DieselConv3D, JsonConv};

const DATE_FORMAT: &'static str = "%Y-%m-%d";

pub trait FromEntry<Entry>: Sized {
    fn from_entry(e: Entry) -> Result<Self, String>;

    fn to_resp<Resp>(e: Entry) -> Resp
    where
        Resp: From<Self>,
        String: AsBackendErrorResp<Resp>,
    {
        match Self::from_entry(e) {
            Ok(json) => json.into(),
            Err(msg) => msg.as_backend_error_resp(),
        }
    }

    fn vec_to_resp<Resp>(v: Vec<Entry>) -> Resp
    where
        Resp: FromIterator<Self>,
        String: AsBackendErrorResp<Resp>,
    {
        let res: Result<Resp, String> = v.into_iter().map(Self::from_entry).collect();
        match res {
            Ok(jsons) => jsons,
            Err(msg) => msg.as_backend_error_resp(),
        }
    }
}

impl<E, T: FromEntry<E>> FromEntry<Option<E>> for swagger::Nullable<T> {
    fn from_entry(e: Option<E>) -> Result<Self, String> {
        Ok(match e {
            Some(e) => swagger::Nullable::Present(T::from_entry(e)?),
            None => swagger::Nullable::Null,
        })
    }
}

macro_rules! geo_conv {
    ($entry:path, $json:ident) => {
        impl FromEntry<$entry> for models::$json {
            fn from_entry(e: $entry) -> Result<Self, String> {
                e.try_to_json().map_err(|e| e.to_string())
            }
        }
        impl FromJson<models::$json> for $entry {
            fn from_json(e: models::$json) -> Result<Self, String> {
                e.try_to_diesel().map_err(|e| e.to_string())
            }
        }
    };
}
geo_conv!(PointZ, PointWithHeight);

pub fn date_to_str(d: chrono::NaiveDate) -> String {
    d.format(DATE_FORMAT).to_string()
}

macro_rules! parse_json {
    ($ty:path, $e:expr) => {
        match <$ty>::from_json($e) {
            Ok(v) => v,
            Err(msg) => return msg.as_bad_request(),
        }
    };
}

pub trait FromJson<Json>: Sized {
    fn from_json(e: Json) -> Result<Self, String>;
}

impl FromJson<String> for chrono::NaiveDate {
    fn from_json(s: String) -> Result<Self, String> {
        Self::parse_from_str(&s, DATE_FORMAT).map_err(|e| e.to_string())
    }
}

impl FromJson<i64> for i64 {
    fn from_json(e: i64) -> Result<Self, String> {
        Ok(e)
    }
}

impl FromJson<i32> for i32 {
    fn from_json(e: i32) -> Result<Self, String> {
        Ok(e)
    }
}

impl<'a> FromJson<FieldFilter<'a>> for FieldFilter<'a> {
    fn from_json(f: FieldFilter<'a>) -> Result<Self, String> {
        Ok(f)
    }
}

impl<J, T: FromJson<J>> FromJson<Option<J>> for Option<T> {
    fn from_json(j: Option<J>) -> Result<Self, String> {
        j.map(FromJson::from_json).transpose()
    }
}

impl<J, T: FromJson<J>> FromJson<swagger::Nullable<J>> for Option<T> {
    fn from_json(j: swagger::Nullable<J>) -> Result<Self, String> {
        Ok(match j {
            Nullable::Null => None,
            Nullable::Present(j) => Some(T::from_json(j)?),
        })
    }
}

impl<JA, A, JB, B> FromJson<(JA, JB)> for (A, B)
where
    A: FromJson<JA>,
    B: FromJson<JB>,
{
    fn from_json((ja, jb): (JA, JB)) -> Result<Self, String> {
        Ok((A::from_json(ja)?, B::from_json(jb)?))
    }
}

pub trait Get<Entry, Json>
where
    Self: From<Json> + NotFoundResponse,
    Json: FromEntry<Entry>,
    String: AsBackendErrorResp<Self> + AsBadRequestResponse<Self>,
{
    fn get<Conn>(conn: &mut Conn, id: Entry::ID) -> Self
    where
        Entry: Readable<Conn>,
        <Entry::B as Backend>::Error: AsBackendErrorResp<Self>,
    {
        match Entry::read_one(conn, id) {
            Ok(Some(entry)) => Json::to_resp(entry),
            Ok(None) => Self::not_found(),
            Err(e) => <Entry::B as Backend>::Error::from(e).as_backend_error_resp(),
        }
    }
}

pub trait GetAll<Entry, Json>
where
    Self: FromIterator<Json>,
    Json: FromEntry<Entry>,
    String: AsBackendErrorResp<Self> + AsBadRequestResponse<Self>,
{
    fn get_all<Conn>(conn: &mut Conn, offset: usize, count: usize) -> Self
    where
        Entry: BatchReadable<Conn>,
        String: AsBackendErrorResp<Self>,
        <Entry::B as Backend>::Error: AsBackendErrorResp<Self>,
    {
        match Entry::read_all(conn, offset, count) {
            Ok(v) => Json::vec_to_resp(v),
            Err(e) => <Entry::B as Backend>::Error::from(e).as_backend_error_resp(),
        }
    }
}

pub trait Search<Entry, Json>
where
    Self: FromIterator<Json>,
    Json: FromEntry<Entry>,
    String: AsBackendErrorResp<Self> + AsBadRequestResponse<Self>,
{
    fn search<'a, Conn, F: 'a>(conn: &mut Conn, filter_json: F, offset: usize, count: usize) -> Self
    where
        Entry: 'a + Searchable<Conn>,
        Entry::Filter<'a>: FromJson<F>,
        <Entry::B as Backend>::Error: AsBackendErrorResp<Self>,
    {
        let filter = parse_json!(Entry::Filter<'a>, filter_json);
        match Entry::search(conn, filter, offset, count) {
            Ok(v) => Json::vec_to_resp(v),
            Err(e) => <Entry::B as Backend>::Error::from(e).as_backend_error_resp(),
        }
    }
}

pub trait Delete<Entry>
where
    Self: Sized + UnitSuccessResponse + NotFoundResponse,
{
    fn delete<Conn>(conn: &mut Conn, id: Entry::ID) -> Self
    where
        Entry: Deletable<Conn>,
        <Entry::B as Backend>::Error: AsBackendErrorResp<Self>,
    {
        match Entry::delete(conn, id) {
            Ok(Some(_)) => Self::succeed(),
            Ok(None) => Self::not_found(),
            Err(e) => <Entry::B as Backend>::Error::from(e).as_backend_error_resp(),
        }
    }
}

pub trait Post<Insert, Json>
where
    Insert: FromJson<Json>,
{
    fn post<Conn, Entry>(conn: &mut Conn, json: Json) -> Self
    where
        Self: From<Entry::ID>,
        Entry: DbEntry<Conn>,
        Insert: Creatable<Conn, Entry> + Debug,
        String: AsBackendErrorResp<Self> + AsBadRequestResponse<Self>,
        <Entry::B as Backend>::Error: AsBackendErrorResp<Self>,
    {
        let insert = parse_json!(Insert, json);
        debug!("Receiving post {:?}", insert);
        match insert.create(conn) {
            Ok(id) => id.into(),
            Err(e) => <Entry::B as Backend>::Error::from(e).as_backend_error_resp(),
        }
    }
}

pub trait Upsert<ChangeSet, Entry, PatchJson, RespJson>
where
    Self: From<RespJson>,
    ChangeSet: FromJson<PatchJson>,
{
    fn upsert<Conn>(conn: &mut Conn, json: PatchJson) -> Self
    where
        Entry: DbEntry<Conn>,
        RespJson: FromEntry<Entry>,
        ChangeSet: Upsertable<Conn, Entry> + Debug,
        String: AsBackendErrorResp<Self> + AsBadRequestResponse<Self>,
        <Entry::B as Backend>::Error: AsBackendErrorResp<Self>,
    {
        let change_set = parse_json!(ChangeSet, json);
        debug!("Receiving upsert patch {:?}", change_set);
        match change_set.upsert(conn) {
            Ok(entry) => RespJson::to_resp(entry),
            Err(e) => <Entry::B as Backend>::Error::from(e).as_backend_error_resp(),
        }
    }
}

pub trait Patch<ChangeSet, Entry, PatchJson, RespJson>
where
    Self: From<RespJson> + NotFoundResponse,
    ChangeSet: FromJson<PatchJson>,
{
    fn patch<Conn>(conn: &mut Conn, id: &Entry::ID, json: PatchJson) -> Self
    where
        Entry: DbEntry<Conn>,
        RespJson: FromEntry<Entry>,
        ChangeSet: Updatable<Conn, Entry> + Debug,
        String: AsBackendErrorResp<Self> + AsBadRequestResponse<Self>,
        <Entry::B as Backend>::Error: AsBackendErrorResp<Self>,
    {
        let change_set = parse_json!(ChangeSet, json);
        debug!("Receiving patch {:?}", change_set);
        match change_set.update(conn, id) {
            Ok(Some(entry)) => RespJson::to_resp(entry),
            Ok(None) => Self::not_found(),
            Err(e) => <Entry::B as Backend>::Error::from(e).as_backend_error_resp(),
        }
    }
}

pub trait DeleteAll<Entry, FilterJson, RespJson>
where
    Self: FromIterator<RespJson>,
    RespJson: FromEntry<Entry>,
    String: AsBackendErrorResp<Self> + AsBadRequestResponse<Self>,
{
    fn delete_all<'a, Conn>(conn: &mut Conn, filter: FilterJson) -> Self
    where
        Entry: BatchDeletable<Conn> + 'a,
        Entry::Filter<'a>: FromJson<FilterJson>,
        <Entry::B as Backend>::Error: AsBackendErrorResp<Self>,
    {
        let filter = parse_json!(Entry::Filter<'a>, filter);
        match Entry::batch_delete(conn, filter) {
            Ok(v) => RespJson::vec_to_resp(v),
            Err(e) => <Entry::B as Backend>::Error::from(e).as_backend_error_resp(),
        }
    }
}
