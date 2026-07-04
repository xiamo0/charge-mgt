//! SetMonitoringBase Request (Block N)
use serde::{Deserialize, Serialize};
use crate::common::MonitoringBaseEnumType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringBaseRequest {
    pub monitoring_base: MonitoringBaseEnumType,
}

pub const ACTION: &str = "SetMonitoringBase";
