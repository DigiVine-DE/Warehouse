pub mod db;
mod filter;
pub mod planting_run;
pub use filter::FieldFilter;

#[cfg(test)]
mod tests {
    use super::{db::FieldPersistent, *};

    use diesel_ext::db_tester::SearchTester;
    use serial_test::serial;

    /*
    We will only check the following fields For convenience.
    |----+-----------+------------+---------------------------|
    | ID | Name      | Fruit      | BBox                      |
    |----+-----------+------------+---------------------------|
    | 35 | Letten    | Chardonnay | 8.089 49.183 8.21 49.429  |
    | 36 | Mütterle  | Chardonnay | 8.089 49.183 8.09 49.184  |
    | 40 | Mütterle  | Dornfelder | 8.105 49.179 8.106 49.181 |
    | 42 | Rosenberg | Dornfelder | 8.09 49.111 8.092 49.112  |
    | 43 | Mütterle  | Dornfelder | 8.072 49.175 8.073 49.177 |
    |----+-----------+------------+---------------------------|
    */
    fn tester() -> SearchTester<FieldPersistent> {
        SearchTester::id_ranges([35, 36, 40, 42, 43])
    }

    #[test]
    #[serial]
    fn field_filter_test() {
        let t = tester();
        t.check(
            (
                FieldFilter::new().with_fuzzy(&vec!["Mütterle"]),
                Some([8.05, 49.1, 8.25, 49.182].into()),
            ),
            vec![40, 43],
        );

        t.check(
            (
                FieldFilter::new().with_fuzzy(&vec!["Rosenberg"]),
                Some([8.05, 49.1, 8.25, 49.182].into()),
            ),
            vec![42],
        );
    }

    #[test]
    #[serial]
    fn field_filter_by_bbox_test() {
        let t = tester();
        let bbox = |a, b, c, d| (FieldFilter::new(), Some([a, b, c, d].into()));
        t.check(bbox(8.05, 49.1, 8.25, 49.5), vec![35, 36, 40, 42, 43]);
        t.check(bbox(8.15, 49.1, 8.25, 49.15), vec![]);
        t.check(bbox(8.05, 49.1, 8.25, 49.182), vec![40, 42, 43]);
    }

    #[test]
    #[serial]
    fn field_filter_by_keywords_test() {
        let t = tester();
        t.check(
            (
                FieldFilter::new().with_fuzzy(&vec!["Letten", "Chardonnay"]),
                None,
            ),
            vec![35, 36],
        );

        t.check(
            (FieldFilter::new().with_fuzzy(&vec!["tt"]), None),
            vec![35, 36, 40, 43],
        );

        t.check(
            (FieldFilter::new().with_fuzzy(&vec!["dorn", "mü"]), None),
            vec![40, 43, 36, 42],
        );

        t.check(
            (FieldFilter::new().with_field_names(&vec!["Letten"]), None),
            vec![35],
        );

        t.check(
            (FieldFilter::new().with_field_names(&vec!["mü"]), None),
            vec![36, 40, 43],
        );

        t.check(
            (
                FieldFilter::new().with_fruit_names(&vec!["Chardonnay"]),
                None,
            ),
            vec![35, 36],
        );

        t.check(
            (
                FieldFilter::new()
                    .with_fuzzy(&vec!["dorn"])
                    .with_field_names(&vec!["mü"])
                    .with_fruit_names(&vec!["Chardonnay"]),
                None,
            ),
            vec![],
        );

        t.check(
            (
                FieldFilter::new()
                    .with_fuzzy(&vec!["mütterle", "dornfelder", "felder"])
                    .with_field_names(&vec!["mü"]),
                None,
            ),
            vec![40, 43, 36],
        );

        t.check(
            (
                FieldFilter::new()
                    .with_fuzzy(&vec!["r"])
                    .with_fruit_names(&vec!["Dornfelder"]),
                None,
            ),
            vec![40, 42, 43],
        );

        t.check(
            (
                FieldFilter::new()
                    .with_fuzzy(&vec!["R"])
                    .with_fruit_names(&vec!["Dornfelder"]),
                None,
            ),
            vec![42, 40, 43],
        );
    }
}
