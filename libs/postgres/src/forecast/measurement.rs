use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeasurementRate {
    Daily,
}

impl From<String> for MeasurementRate {
    fn from(_string: String) -> Self {
        // change this if eventually another measurement rate appears
        MeasurementRate::Daily
    }
}

impl From<MeasurementRate> for String {
    fn from(_rate: MeasurementRate) -> Self {
        // change this if eventually another measurement rate appears
        "Daily".into()
    }
}
