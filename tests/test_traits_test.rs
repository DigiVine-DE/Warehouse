use diesel::PgConnection;
use diesel_ext::{
    db_backend::{DatabaseError, DbEntry, Readable},
    db_tester::{Mockable, ReadTestable},
};
use digivine_api_postgres::Relational;
use frunk::LabelledGeneric;
use macros_lib::{gen_read_tests, Mockable};

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Mockable, LabelledGeneric))]
pub struct Entry {
    pub entry_id: i64,
    pub import_key: String,
    pub timestamp: chrono::NaiveDateTime,
}

impl Entry {
    fn allowed_ids() -> Vec<i64> {
        vec![1, 4, 5, 6, 99]
    }
}

impl DbEntry<PgConnection> for Entry {
    type B = Relational;
    type ID = i64;
}

#[gen_read_tests(id = self.entry_id, testable_ids = [1, 4..=6, 99])]
impl Readable<PgConnection> for Entry {
    fn read_one(_: &mut PgConnection, entry_id: Self::ID) -> Result<Option<Self>, DatabaseError> {
        if Entry::allowed_ids().contains(&entry_id) {
            Ok(Some(Self {
                entry_id,
                import_key: "test_key".to_string(),
                timestamp: Mockable::mock_new(&mut rand::thread_rng()),
            }))
        } else {
            Ok(None)
        }
    }
}

#[test]
fn testable_ids_test() {
    assert_eq!(Entry::testable_ids(), Entry::allowed_ids())
}
