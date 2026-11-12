table! {
    use diesel::sql_types::*;

    jkishapedata (jki_shape_id) {
        jki_shape_id -> Int4,
        id -> Float8,
        gridcode -> Float8,
        orig_fid -> Float8,
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

allow_tables_to_appear_in_same_query!(jkishapedata, spatial_ref_sys,);
