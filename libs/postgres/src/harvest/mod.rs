pub mod harvest_complete;
pub mod harvest_task;

#[cfg(test)]
mod tests {
    use super::{harvest_complete::NestedHarvestComplete, harvest_task::HarvestTaskPersistent};
    use diesel_ext::db_tester::{ReadTestable, SearchTester};

    fn helper(bbox: [f64; 4], want_ids: Vec<i64>) {
        SearchTester::<HarvestTaskPersistent>::new().check(Some(bbox.into()), want_ids.clone());
        SearchTester::<NestedHarvestComplete>::new().check(Some(bbox.into()), want_ids.clone());
    }

    #[test]
    fn harvest_search_test() {
        helper([99999., 99999., 99999., 99999.], vec![]);

        helper(
            [
                10.874598669868703,
                51.611413288880755,
                10.874598669868703,
                51.611413288880755,
            ],
            vec![1],
        );

        helper(
            [
                8.232067521400438,
                51.78187345248659,
                9.979058310268158,
                52.47655797391582,
            ],
            vec![9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
        );

        helper(
            [
                8.323062989232284,
                51.17359336486274,
                11.436250107453247,
                52.422332442967445,
            ],
            HarvestTaskPersistent::testable_ids(),
        )
    }
}
