//! SetMonitoringBase Request (Block N)
use crate::common::MonitoringBaseEnumType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringBaseRequest {
    pub monitoring_base: MonitoringBaseEnumType,
}

pub const ACTION: &str = "SetMonitoringBase";
