use macros_lib::Model;

#[test]
fn single_field_test() {
    #[allow(dead_code)]
    #[derive(Debug, PartialEq, Model)]
    pub struct InlineResponse201 {
        pub id: Option<i64>,
    }
    let id_option: Option<i64> = Some(1);
    let want = InlineResponse201 { id: id_option };
    assert_eq!(InlineResponse201::from(id_option), want);

    let id: i64 = 1;
    let want = InlineResponse201 { id: Some(id) };
    assert_eq!(InlineResponse201::from(id), want);
}

#[test]
fn multi_fields_test() {
    #[derive(Debug, Clone, PartialEq, Model)]
    pub struct ForecastMeasurement {
        pub id: Option<i64>,
        pub time: chrono::DateTime<chrono::Utc>,
        pub values: serde_json::Value,
    }
    let id = Some(1);
    let time = chrono::DateTime::default();
    let values = serde_json::Value::Null;
    let want = ForecastMeasurement {
        id,
        time,
        values: values.clone(),
    };
    assert_eq!(ForecastMeasurement::from((id, time, values)), want);
}
