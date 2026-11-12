table! {
    use diesel::sql_types::*;

    jkilidardata (jki_lidar_id) {
        jki_lidar_id -> Int4,
        longitude -> Float8,
        latitude -> Float8,
        altitude -> Float8,
        gpstime -> Nullable<Float8>,
        intensity -> Nullable<Int4>,
        return_number -> Nullable<Int4>,
        number_of_returns -> Nullable<Int4>,
        scan_direction_flag -> Nullable<Int4>,
        edge_of_flight_line -> Nullable<Int4>,
        classification -> Nullable<Int4>,
        synthetic -> Nullable<Bool>,
        keypoint -> Nullable<Bool>,
        withheld -> Nullable<Bool>,
        scan_angle_rank -> Nullable<Int4>,
        user_data -> Nullable<Int4>,
        point_source_id -> Nullable<Int4>,
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

allow_tables_to_appear_in_same_query!(jkilidardata, spatial_ref_sys,);
