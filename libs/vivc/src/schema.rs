// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;

    vivc (id) {
        id -> Int4,
        cultivar_name -> Varchar,
        prime_name -> Varchar,
        variety_number -> Int4,
        species -> Nullable<Varchar>,
        color -> Nullable<Varchar>,
        country_or_region -> Nullable<Varchar>,
    }
}
