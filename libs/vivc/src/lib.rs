#![feature(generic_associated_types)]

extern crate openssl;
#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
extern crate log;

pub mod db;
mod schema;

use diesel_ext::db_backend::{Backend, DatabaseError};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VivcLibError {
    #[error("error during database communication: {0}")]
    DBError(#[from] DatabaseError),
}

pub struct Vivc;
impl Backend for Vivc {
    type Error = VivcLibError;

    const NAME: &'static str = "Vivc";
    const SOURCE: EmbeddedMigrations = embed_migrations!();
    const MOCK_DATA: Option<EmbeddedMigrations> = None;
}

#[cfg(test)]
mod tests {
    use crate::db::{CultivarFilter, VivcPersistent};
    use diesel_ext::db_tester::SearchTester;
    use serial_test::serial;

    /*
        We will only check the following cultivars For convenience.
    |----|--------------|----------------|-------|-------------------|------------------------------|
    | ID | prime_name   | cultivar_name  | color | country_or_region | species                      |
    |----|--------------|----------------|-------|-------------------|------------------------------|
    | 10 | CUCCUAU      | CUCCUAU DI ULA | BLANC | ITALY             | VITIS VINIFERA LINNÉ ...     |
    | 12 | CUCCUAU      | CUCCUVAJU      | BLANC | ITALY             | VITIS VINIFERA LINNÉ ...     |
    | 20 | CORNACCHIONE | CORNACCHIONE   | ROSE  | ITALY             | VITIS VINIFERA LINNÉ ...     |
    | 40 | MESSIAS      | MESSIAS        | NOIR  | HUNGARY           | VITIS VINIFERA LINNÉ ...     |
    | 50 | SIMONA       | SIMONA         | NOIR  | SERBIA            | VITIS INTERSPECIFIC CROSSING |
    | 52 | LEONTINA     | RADMILOVAC ZC  | NOIR  | SERBIA            | VITIS INTERSPECIFIC CROSSING |
    |----|--------------|----------------|-------|-------------------|------------------------------|
                    */

    #[test]
    #[serial]
    fn cultivar_filter_test() {
        let t = SearchTester::<VivcPersistent>::id_ranges([10, 12, 20, 40, 50, 52]);
        t.check(
            CultivarFilter::default().with_species(&vec!["vitis"]),
            vec![10, 12, 20, 40, 50, 52],
        );
        t.check(
            CultivarFilter::default().with_prime_name(&vec!["cucc"]),
            vec![10, 12],
        );
        t.check(
            CultivarFilter::default().with_cultivar_name(&vec!["cucc", "di"]),
            vec![10],
        );
        t.check(
            CultivarFilter::default().with_species(&vec!["INTERSPECIFIC"]),
            vec![50, 52],
        );
        t.check(
            CultivarFilter::default()
                .with_species(&vec!["vinifera"])
                .with_region(&vec!["ITALY"]),
            vec![10, 12, 20],
        );
        t.check(
            CultivarFilter::default().with_color(&vec!["nOir"]),
            vec![40, 50, 52],
        );
        t.check(
            CultivarFilter::default().with_color(&vec!["NOIR", "ROSE"]),
            vec![],
        );
        t.check(
            CultivarFilter::default().with_region(&vec!["hun"]),
            vec![40],
        );
        t.check(
            CultivarFilter::default()
                .with_region(&vec!["Italy"])
                .with_species(&vec!["linn"])
                .with_color(&vec!["bl"])
                .with_prime_name(&vec!["cucc"])
                .with_cultivar_name(&vec!["cucc", "di"]),
            vec![10],
        );
    }
}
