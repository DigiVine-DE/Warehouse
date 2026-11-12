use macros_lib::Mockable;

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(test, derive(Mockable))]
pub struct Entry {
    #[mockable_const(1+4)]
    pub id: i64,
    pub import_key: String,
    pub timestamp: chrono::NaiveDateTime,
}

#[test]
fn mockable_derive_test() {
    use diesel_ext::db_tester::Mockable;
    let mut rng = rand::thread_rng();
    let entry = Entry::mock_new(&mut rng);
    for _ in 0..5 {
        let e = Entry::mock_new(&mut rng);
        assert_eq!(e.id, 5);
        assert_ne!(entry, e);
    }
}
