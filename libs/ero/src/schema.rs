table! {
    use diesel::sql_types::*;


    erodata (ero_id) {
        ero_id -> Int4,
        lat -> Float8,
        lon -> Float8,
        ndvi_corrected -> Float8,
        time -> Timestamp,
        ndvi_raw -> Float8,
    }
}

table! {
    use diesel::sql_types::*;


    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(erodata, spatial_ref_sys,);
