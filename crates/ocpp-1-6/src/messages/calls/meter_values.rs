//! MeterValues 消息及处理器

use super::super::confs::meter_values_conf::MeterValuesConfirmation;
use crate::common::meter_value::MeterValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeterValuesRequest {
    pub connector_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<i32>,
    pub meter_value: Vec<MeterValue>,
}

pub trait MeterValuesHandler: Send + Sync {
    fn handle(&self, req: MeterValuesRequest) -> MeterValuesConfirmation;
}

pub struct DefaultMeterValuesHandler;

impl Default for DefaultMeterValuesHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultMeterValuesHandler {
    pub fn new() -> Self {
        Self
    }
}

impl MeterValuesHandler for DefaultMeterValuesHandler {
    fn handle(&self, _req: MeterValuesRequest) -> MeterValuesConfirmation {
        MeterValuesConfirmation
    }
}
