use crate::{
    db_backend::{
        Backend, BatchReadable, Creatable, DatabaseError, DbConnector, DbConnectorInitError,
        DbEntry, Deletable, Readable, Searchable, Updatable,
    },
    GeoBBox2D,
};
use chrono::{Days, Local, NaiveDate, NaiveDateTime};
use diesel::{Connection, PgConnection};
use postgis_diesel::gps::{LineString, MultiPolygon, Point, PointZ, Polygon};
use rand::{distributions::Bernoulli, prelude::Distribution, rngs::ThreadRng, thread_rng, Rng};

#[derive(Clone, Debug)]
pub struct DbTester {
    db_url: String,
}

impl Default for DbTester {
    fn default() -> Self {
        Self {
            db_url: Self::test_url(),
        }
    }
}

impl DbTester {
    /// Order of reading values:
    /// 1. env var of name `DIGIVINE_TEST_DB_URL`
    /// 2. `DB_TEST_URL` of file `.env`
    /// 3. postgres://digivine:digivine@postgres/digivine
    pub fn test_url() -> String {
        std::env::var("DIGIVINE_TEST_DB_URL").unwrap_or_else(|_| {
            dotenv::dotenv().ok();
            // Set DB_TEST_URL in .env file
            dotenv::var("DB_TEST_URL")
                .unwrap_or_else(|_| "postgres://digivine:digivine@postgres/digivine".to_string())
        })
    }

    pub fn test_connection(&self) -> PgConnection {
        let mut conn = self.establish_connection().unwrap();
        conn.begin_test_transaction()
            .expect("could not begin test transaction");
        conn
    }
}

impl DbConnector for DbTester {
    type Conn = PgConnection;

    fn new(db_url: &str) -> Result<Self, DbConnectorInitError> {
        Ok(Self {
            db_url: db_url.to_string(),
        })
    }

    fn establish_connection(&self) -> Result<Self::Conn, DatabaseError> {
        PgConnection::establish(&self.db_url).map_err(DatabaseError::from)
    }
}

pub trait ReadTestable
where
    Self: DbEntry<PgConnection> + Readable<PgConnection>,
    Self::ID: std::fmt::Debug + Clone,
{
    const NAME: &'static str;

    fn id(&self) -> Self::ID;

    fn testable_ids() -> Vec<Self::ID>;
    fn missing_id() -> Self::ID;

    fn random_testable_ids() -> Vec<Self::ID> {
        let sample_size = 50;
        let all_ids = Self::testable_ids();
        let len = all_ids.len();
        if len < sample_size {
            return all_ids;
        }
        let d = Bernoulli::from_ratio(sample_size as u32, len as u32).unwrap();
        let res = all_ids
            .into_iter()
            .zip(d.sample_iter(&mut rand::thread_rng()))
            .filter_map(|(id, hit): (Self::ID, bool)| if hit { Some(id) } else { None })
            .collect();
        log::debug!("ids for testing: {:?}", res);
        res
    }

    fn test_conn() -> PgConnection {
        Self::B::db_tester().test_connection()
    }

    fn opt_read(conn: &mut PgConnection, id: &Self::ID) -> Option<Self> {
        Self::read_one(conn, id.clone()).unwrap_or_else(|err| {
            panic!(
                "Error while getting entry [{}] with id {:?} in {} backend: {err}",
                Self::NAME,
                id,
                Self::B::NAME
            )
        })
    }

    fn must_read(conn: &mut PgConnection, id: &Self::ID) -> Self {
        Self::opt_read(conn, id).unwrap_or_else(|| {
            panic!(
                "could not find entry [{}] with id {:?} in {} backend",
                Self::NAME,
                id,
                Self::B::NAME
            )
        })
    }

    fn read_test() {
        let mut conn = Self::test_conn();
        log::debug!("[{}] read_test", Self::NAME);
        for id in Self::random_testable_ids() {
            let entry = Self::must_read(&mut conn, &id);
            assert_eq!(entry.id(), id, "Entry with wrong id has benn read");
        }
        let missing_id = Self::missing_id();
        assert!(
            Self::opt_read(&mut conn, &missing_id).is_none(),
            "entry with missing id {:?} is found",
            missing_id
        );
    }
}

pub trait DeleteTestable
where
    Self: ReadTestable + Deletable<PgConnection>,
    Self::ID: std::fmt::Debug + Clone,
{
    fn opt_delete(conn: &mut PgConnection, id: &Self::ID) -> Option<Self::ID> {
        Self::delete(conn, id.clone()).unwrap_or_else(|err| {
            panic!(
                "Error while deleting entry with id {:?} in {} backend: {err}",
                id,
                Self::B::NAME
            )
        })
    }

    fn must_delete(conn: &mut PgConnection, id: &Self::ID) -> Self::ID {
        Self::opt_delete(conn, id).unwrap_or_else(|| {
            panic!(
                "could not find entry [{}] with id {:?} in {} backend",
                Self::NAME,
                id,
                Self::B::NAME
            )
        })
    }

    fn delete_test() {
        let mut conn = Self::test_conn();
        log::debug!("[{}] delete_test", Self::NAME);
        for id in Self::random_testable_ids() {
            assert_eq!(Self::must_delete(&mut conn, &id), id);
        }
        let missing_id = Self::missing_id();
        assert!(
            Self::opt_delete(&mut conn, &missing_id).is_none(),
            "entry with missing id {:?} is deleted",
            missing_id
        );
    }
}

pub trait BatchReadTestable
where
    Self: ReadTestable + BatchReadable<PgConnection> + PartialEq + std::fmt::Debug,
    Self::ID: std::fmt::Debug + Clone,
{
    const OFFSET: usize;
    const LIMIT: usize;

    fn random_range() -> (usize, usize) {
        let sample_size = 50;
        if Self::LIMIT <= sample_size {
            return (Self::OFFSET, Self::LIMIT);
        }
        let offset = thread_rng().gen_range(Self::OFFSET..=Self::LIMIT - sample_size);
        (offset, sample_size)
    }

    fn batch_read_test() {
        let mut conn = Self::test_conn();
        log::debug!("[{}] batch_read_test", Self::NAME);
        let (offset, limit) = Self::random_range();
        let entries = Self::read_all(&mut conn, offset, limit).unwrap_or_else(|err| {
            panic!(
                "Error while batch reading entries in {} backend: {err}",
                Self::B::NAME
            )
        });
        assert_eq!(entries.len(), limit);
        for entry in entries {
            let id = entry.id();
            let read_entry = Self::must_read(&mut conn, &id);
            assert_eq!(
                entry, read_entry,
                "the read_one entry is not the same as the read_all entry"
            );
        }
    }
}

#[derive(Debug)]
pub struct SearchTester<Entry: DbEntry<PgConnection>> {
    id_range: Option<Vec<Entry::ID>>,
}

impl<Entry: DbEntry<PgConnection>> SearchTester<Entry> {
    pub fn new() -> Self {
        SearchTester { id_range: None }
    }

    pub fn id_ranges(r: impl IntoIterator<Item = Entry::ID>) -> Self {
        Self {
            id_range: Some(r.into_iter().collect()),
        }
    }

    pub fn check<'a>(&self, filter: Entry::Filter<'a>, want_ids: Vec<Entry::ID>)
    where
        Entry: ReadTestable + Searchable<PgConnection> + PartialEq + std::fmt::Debug + 'a,
        Entry::ID: std::fmt::Debug + Clone + PartialEq,
        Entry::Filter<'a>: std::fmt::Debug + Clone,
    {
        let mut conn = Entry::test_conn();
        let got = Entry::search(&mut conn, filter.clone(), 0, 100).unwrap_or_else(|err| {
            panic!(
                " [{}] Error while searching entries with filter {:?} in {} backend: {}",
                Entry::NAME,
                filter,
                Entry::B::NAME,
                err,
            )
        });
        let got_filtered_ids: Vec<_> = got
            .into_iter()
            .map(|e| e.id())
            .filter(|id| match &self.id_range {
                Some(r) => r.contains(id),
                None => true,
            })
            .collect();
        assert_eq!(
            got_filtered_ids, want_ids,
            "the ids of found entries with filter {:?} do not match expected ids",
            &filter,
        );
    }
}

pub trait UpdateTestable<Entry>
where
    Self: Updatable<PgConnection, Entry> + Sized + Mockable + std::fmt::Debug + Clone,
    Entry: ReadTestable + PartialEq + Clone + std::fmt::Debug,
    Entry::ID: std::fmt::Debug + Clone,
{
    fn update_test() {
        let mut conn = Entry::test_conn();
        log::debug!("[{}] update_test", Entry::NAME);
        let mut rng = rand::thread_rng();
        for id in Entry::random_testable_ids() {
            let entry = Entry::must_read(&mut conn, &id);
            let entry_update = Self::mock_new(&mut rng);
            entry_update
                .clone()
                .update(&mut conn, &id)
                .unwrap_or_else(|err| {
                    panic!(
                        "Error while updating entry from {:?} to {:?} in {} backend: {}",
                        entry,
                        entry_update,
                        Entry::B::NAME,
                        err,
                    )
                })
                .unwrap_or_else(|| {
                    panic!(
                        "the entry to update cannot be found: {:?} in {} backend",
                        entry_update,
                        Entry::B::NAME
                    )
                });
            let new_entry = Entry::must_read(&mut conn, &id);
            assert_ne!(
                entry, new_entry,
                "new entry should be different after update via patch: {:?}",
                entry_update
            );
        }
    }
}

pub trait CreateTestable<Entry>
where
    Self: Creatable<PgConnection, Entry> + Mockable + Sized + std::fmt::Debug + Clone,
    Entry: ReadTestable + PartialEq + std::fmt::Debug,
    Entry::ID: std::fmt::Debug + Clone,
{
    fn create_test() {
        let t = Entry::B::db_tester();
        log::debug!("[{}] create_test", Entry::NAME);
        let mut rng = rand::thread_rng();
        for _ in 0..3 {
            let mut conn = t.test_connection(); // reconnect to abort previous changes
            let entry_insert = Self::mock_new(&mut rng);
            let id = entry_insert
                .clone()
                .create(&mut conn)
                .unwrap_or_else(|err| {
                    panic!(
                        "Error while inserting entry via {:?} in {} backend: {err}",
                        entry_insert,
                        Entry::B::NAME
                    )
                });
            assert!(
                !Entry::testable_ids().contains(&id),
                "the id {:?} of the new inserted entry is in testable ids",
                id
            );
            Entry::must_read(&mut conn, &id);
        }
    }
}

pub trait Mockable: Sized {
    fn mock_new(rng: &mut ThreadRng) -> Self;
}

impl<T: Mockable> Mockable for Option<T> {
    fn mock_new(rng: &mut ThreadRng) -> Self {
        Some(T::mock_new(rng))
    }
}

impl<T: Mockable> Mockable for Vec<T> {
    fn mock_new(rng: &mut ThreadRng) -> Self {
        let vec_len: usize = rng.gen_range(0..=5);
        let mut res = Vec::new();
        res.resize_with(vec_len, || T::mock_new(rng));
        res
    }
}

impl Mockable for NaiveDate {
    fn mock_new(rng: &mut ThreadRng) -> Self {
        let base = NaiveDate::from_ymd_opt(1000, 1, 1).unwrap();
        let days = rng.gen_range(0..40000);
        base + Days::new(days)
    }
}

impl Mockable for i64 {
    fn mock_new(rng: &mut ThreadRng) -> Self {
        rng.gen_range(1..i64::MAX)
    }
}

impl Mockable for i32 {
    fn mock_new(rng: &mut ThreadRng) -> Self {
        rng.gen_range(1..i32::MAX)
    }
}

impl Mockable for i16 {
    fn mock_new(rng: &mut ThreadRng) -> Self {
        rng.gen_range(1..i16::MAX)
    }
}

impl Mockable for f64 {
    fn mock_new(rng: &mut ThreadRng) -> Self {
        rng.gen()
    }
}

impl Mockable for String {
    fn mock_new(rng: &mut ThreadRng) -> Self {
        format!("mock_{}", rng.gen::<u8>())
    }
}

impl Mockable for chrono::NaiveDateTime {
    fn mock_new(rng: &mut ThreadRng) -> Self {
        // postgres only supports precesion up to millisecond
        let miils = Local::now().timestamp_millis();
        let delta: i64 = rng.gen_range(-500..500);
        NaiveDateTime::from_timestamp_millis(miils + delta).unwrap()
    }
}

impl Mockable for Point {
    fn mock_new(rng: &mut ThreadRng) -> Self {
        Point::new(rng.gen(), rng.gen())
    }
}

impl Mockable for PointZ {
    fn mock_new(rng: &mut ThreadRng) -> Self {
        PointZ::new(rng.gen(), rng.gen(), rng.gen())
    }
}

impl Mockable for Polygon {
    fn mock_new(rng: &mut ThreadRng) -> Self {
        let mut points: Vec<_> = std::iter::repeat_with(|| Point::mock_new(rng))
            .take(3)
            .collect();
        points.push(points[0]); // make line string close
        let ls: LineString = points.into_iter().collect();
        std::iter::once(ls).collect()
    }
}

impl Mockable for MultiPolygon {
    fn mock_new(rng: &mut ThreadRng) -> Self {
        let len = rng.gen_range(1..4);
        std::iter::repeat_with(|| Polygon::mock_new(rng))
            .take(len)
            .collect()
    }
}

impl Mockable for GeoBBox2D {
    fn mock_new(rng: &mut ThreadRng) -> Self {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        [x / 2., y / 2., x, y].into()
    }
}
