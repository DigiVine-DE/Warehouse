use diesel::{
    connection::LoadConnection,
    migration::MigrationSource,
    pg::Pg,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
    PgConnection,
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use thiserror::Error;

#[cfg(feature = "test-utils")]
use crate::db_tester::DbTester;

pub trait DbConnector: Sized + Send + Clone + 'static {
    type Conn: LoadConnection<Backend = Pg> + MigrationHarness<Pg>;

    fn new(db_url: &str) -> Result<Self, DbConnectorInitError>;
    fn establish_connection(&self) -> Result<Self::Conn, DatabaseError>;

    fn migrate<S: MigrationSource<Pg>>(&self, s: S) -> Result<(), DatabaseError> {
        let mut con = self.establish_connection()?;
        let log = con
            .run_pending_migrations(s)
            .map_err(|e| DatabaseError::Migration(e))?;
        for v in log {
            log::info!("{:?} done", v);
        }
        Ok(())
    }
}

pub trait Backend {
    type Error: std::error::Error + From<DatabaseError>;
    const NAME: &'static str;
    const SOURCE: EmbeddedMigrations;
    const MOCK_DATA: Option<EmbeddedMigrations>;

    fn new<Conn: DbConnector>(
        db_url: &str,
        with_mock_data: bool,
    ) -> std::result::Result<Conn, Self::Error> {
        log::info!("Using as {} Backend: {}", Self::NAME, db_url);
        let c = Conn::new(db_url).map_err(DatabaseError::from)?;
        log::info!("Migrating {} Backend...", Self::NAME);
        c.migrate(Self::SOURCE)?;
        match (Self::MOCK_DATA, with_mock_data) {
            (Some(mock), true) => {
                log::info!("Adding mock data to {} Backend...", Self::NAME);
                c.migrate(mock)?
            }
            (None, true) => log::warn!("Backend {} doesn't have mock data", Self::NAME),
            _ => (),
        };
        log::info!("{} Backend Migration finished", Self::NAME);
        Ok(c)
    }

    #[cfg(feature = "test-utils")]
    fn db_tester() -> DbTester {
        // ignore error as log can only be init once
        let _ = simplelog::SimpleLogger::init(
            simplelog::LevelFilter::Debug,
            simplelog::Config::default(),
        );
        Self::new(&DbTester::test_url(), true).unwrap_or_else(|err| {
            panic!(
                "could not init db tester of {} backend: {}",
                Self::NAME,
                err
            )
        })
    }
}

#[derive(Clone)]
pub struct PoolBackend {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DbConnector for PoolBackend {
    type Conn = PooledConnection<ConnectionManager<PgConnection>>;

    fn new(db_url: &str) -> Result<Self, DbConnectorInitError> {
        let manager = ConnectionManager::new(db_url);
        let pool = Pool::builder().build(manager)?;
        Ok(Self { pool })
    }

    fn establish_connection(&self) -> Result<Self::Conn, DatabaseError> {
        self.pool
            .get()
            .map_err(DbConnectorInitError::from)
            .map_err(DatabaseError::from)
    }
}

#[derive(Error, Debug)]
pub enum DbConnectorInitError {
    #[error("Could not init DB connection pool: {0}")]
    PoolError(#[from] PoolError),
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection error: {0}")]
    DbConnectorInitError(#[from] DbConnectorInitError),
    #[error("could not connect to DB: {0}")]
    CouldNotConnect(#[from] diesel::ConnectionError),
    #[error("Error in connection pool: {0}")]
    PoolError(#[from] PoolError),
    #[error("could not migrate: {0}")]
    Migration(Box<dyn std::error::Error + Send + Sync>),
    #[error("could not perform operation: {0}")]
    OperationError(#[from] diesel::result::Error),
}

pub trait DbEntry<Conn>: Sized {
    type B: Backend;
    type ID: Eq;
}

pub trait Readable<Conn>: DbEntry<Conn> {
    fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError>;
}

pub trait BatchReadable<Conn>: DbEntry<Conn> {
    fn read_all(c: &mut Conn, offset: usize, count: usize) -> Result<Vec<Self>, DatabaseError>;
}

pub trait Searchable<Conn>: DbEntry<Conn> {
    /// The filter must implement `Default` and the default value will not filter any entries.
    type Filter<'a>: Default
    where
        Self: 'a;

    fn search(
        c: &mut Conn,
        filter: Self::Filter<'_>,
        offset: usize,
        count: usize,
    ) -> Result<Vec<Self>, DatabaseError>;
}

pub trait Deletable<Conn>: DbEntry<Conn> {
    fn delete(c: &mut Conn, id: Self::ID) -> Result<Option<Self::ID>, DatabaseError>;
}

pub trait BatchDeletable<Conn>: DbEntry<Conn> {
    type Filter<'a>
    where
        Self: 'a;

    fn batch_delete(c: &mut Conn, filter: Self::Filter<'_>) -> Result<Vec<Self>, DatabaseError>;
}

pub trait Creatable<Conn, Entry: DbEntry<Conn>> {
    fn create(self, c: &mut Conn) -> Result<Entry::ID, DatabaseError>;
}

pub trait Updatable<Conn, Entry: DbEntry<Conn>> {
    fn update(self, c: &mut Conn, id: &Entry::ID) -> Result<Option<Entry>, DatabaseError>;
}

pub trait Upsertable<Conn, Entry: DbEntry<Conn>> {
    fn upsert(self, c: &mut Conn) -> Result<Entry, DatabaseError>;
}
