// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    fields (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        station_id -> Nullable<Int8>,
        field_number -> Nullable<Varchar>,
        field_size -> Nullable<Float8>,
        fruit_name -> Nullable<Varchar>,
        geom -> Nullable<Geometry>,
        bounding_box -> Geometry,
        historical_station_id -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    forecast (id) {
        id -> Int8,
        group_id -> Int8,
        station_id -> Int8,
        station -> Varchar,
        field_ids -> Array<Int8>,
        model_id -> Int8,
        model -> Varchar,
        query_timestamp -> Timestamp,
        time_span_start -> Date,
        time_span_end -> Date,
        measurement_rate -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    forecast_measurement (id) {
        id -> Int8,
        forecast_id -> Int8,
        time -> Timestamp,
        value -> Jsonb,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    harvestcomplete (task_id) {
        task_id -> Int8,
        task_name -> Varchar,
        customer_name -> Varchar,
        farm_name -> Nullable<Varchar>,
        fruit_group_name -> Varchar,
        fruit_name -> Varchar,
        plain_name -> Varchar,
        plain_size -> Int8,
        yield_total_mass -> Nullable<Int8>,
        total_area -> Nullable<Int8>,
        effective_total_distance -> Nullable<Int8>,
        effective_total_time -> Nullable<Int8>,
        geom -> Nullable<Geometry>,
        average_tartaric_acid -> Nullable<Int8>,
        average_malic_acid -> Nullable<Int8>,
        average_glucose -> Nullable<Int8>,
        average_fructose -> Nullable<Int8>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    harvestmeasurement (id) {
        id -> Int8,
        task_id -> Int8,
        acid -> Int8,
        sugar -> Int8,
        spectral -> Nullable<Varchar>,
        geom -> Nullable<Geometry>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    harvesttask (task_id) {
        task_id -> Int8,
        task_name -> Varchar,
        customer_name -> Varchar,
        farm_name -> Nullable<Varchar>,
        fruit_group_name -> Varchar,
        fruit_name -> Varchar,
        plain_name -> Varchar,
        plain_size -> Int8,
        yield_total_mass -> Nullable<Int8>,
        total_area -> Nullable<Int8>,
        effective_total_distance -> Nullable<Int8>,
        effective_total_time -> Nullable<Int8>,
        geom -> Geometry,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    plant (id) {
        id -> Int8,
        planting_run_id -> Int8,
        import_key -> Varchar,
        timestamp -> Timestamp,
        height -> Float8,
        deviation -> Float8,
        speed -> Float8,
        plant_distance -> Float8,
        row_distance -> Float8,
        row_number -> Int4,
        geo_point -> Geometry,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    planting_run (id) {
        id -> Int8,
        user_id -> Int8,
        terminal_id -> Int8,
        field_id -> Int8,
        bounding_box -> Nullable<Geometry>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    quality_measurement (id) {
        id -> Int8,
        plant_id -> Int8,
        timestamp -> Timestamp,
        device -> Varchar,
        measurement_uuid -> Varchar,
        tartaric_acid -> Float8,
        malic_acid -> Float8,
        fructose -> Float8,
        glucose -> Float8,
        geo_point -> Geometry,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    spray_plantprotection (id) {
        id -> Int8,
        task_id -> Nullable<Int8>,
        complete_id -> Nullable<Int8>,
        name -> Varchar,
        amount -> Int8,
        unit -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    spraycomplete (task_id) {
        task_id -> Int8,
        task_name -> Varchar,
        customer_name -> Varchar,
        farm_name -> Nullable<Varchar>,
        fruit_group_name -> Varchar,
        fruit_name -> Varchar,
        plain_name -> Varchar,
        plain_size -> Int8,
        application_total_volume -> Nullable<Int8>,
        total_area -> Nullable<Int8>,
        effective_total_distance -> Nullable<Int8>,
        effective_total_time -> Nullable<Int8>,
        geom -> Geometry,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    spraytask (task_id) {
        task_id -> Int8,
        task_name -> Varchar,
        customer_name -> Varchar,
        farm_name -> Nullable<Varchar>,
        fruit_group_name -> Varchar,
        fruit_name -> Varchar,
        plain_name -> Varchar,
        plain_size -> Int8,
        geom -> Geometry,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    weather_data (station_id, measurement_date) {
        station_id -> Int4,
        measurement_date -> Date,
        fx -> Nullable<Float8>,
        fx_qn -> Nullable<Int2>,
        fm -> Nullable<Float8>,
        fm_qn -> Nullable<Int2>,
        rsk -> Nullable<Float8>,
        rsk_qn -> Nullable<Int2>,
        rskf -> Nullable<Int4>,
        sdk -> Nullable<Float8>,
        sdk_qn -> Nullable<Int2>,
        shk_tag -> Nullable<Float8>,
        shk_tag_qn -> Nullable<Int2>,
        nm -> Nullable<Float8>,
        nm_qn -> Nullable<Int2>,
        vpm -> Nullable<Float8>,
        vpm_qn -> Nullable<Int2>,
        pm -> Nullable<Float8>,
        pm_qn -> Nullable<Int2>,
        tmk -> Nullable<Float8>,
        tmk_qn -> Nullable<Int2>,
        upm -> Nullable<Float8>,
        upm_qn -> Nullable<Int2>,
        txk -> Nullable<Float8>,
        txk_qn -> Nullable<Int2>,
        tnk -> Nullable<Float8>,
        tnk_qn -> Nullable<Int2>,
        tgk -> Nullable<Float8>,
        tgk_qn -> Nullable<Int2>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    weather_station (station_id) {
        station_id -> Int4,
        station_name -> Nullable<Text>,
        station_bundesland -> Nullable<Text>,
        station_geo_point_z -> Nullable<Geometry>,
    }
}

diesel::joinable!(fields -> weather_station (historical_station_id));
diesel::joinable!(forecast_measurement -> forecast (forecast_id));
diesel::joinable!(harvestmeasurement -> harvestcomplete (task_id));
diesel::joinable!(plant -> planting_run (planting_run_id));
diesel::joinable!(planting_run -> fields (field_id));
diesel::joinable!(quality_measurement -> plant (plant_id));
diesel::joinable!(spray_plantprotection -> spraycomplete (complete_id));
diesel::joinable!(spray_plantprotection -> spraytask (task_id));
diesel::joinable!(weather_data -> weather_station (station_id));

diesel::allow_tables_to_appear_in_same_query!(
    fields,
    forecast,
    forecast_measurement,
    harvestcomplete,
    harvestmeasurement,
    harvesttask,
    plant,
    planting_run,
    quality_measurement,
    spray_plantprotection,
    spraycomplete,
    spraytask,
    weather_data,
    weather_station,
);
