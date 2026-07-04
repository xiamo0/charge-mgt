//! MeterValues Request (Block J)
use crate::common::MeterValueType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesRequest {
    pub evse_id: i32,
    pub meter_value: Vec<MeterValueType>,
}

pub const ACTION: &str = "MeterValues";
