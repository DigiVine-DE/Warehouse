// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    terrain (id) {
        id -> Int8,
        description -> Varchar,
        field_id -> Nullable<Int8>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    terrain_point (id, x, y) {
        id -> Int8,
        x -> Int4,
        y -> Int4,
        geo_point_z -> Geometry,
    }
}

diesel::joinable!(terrain_point -> terrain (id));

diesel::allow_tables_to_appear_in_same_query!(terrain, terrain_point,);
