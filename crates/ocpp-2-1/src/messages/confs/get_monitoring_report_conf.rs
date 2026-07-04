//! GetMonitoringReport Confirmation (Block N)
use serde::{Deserialize, Serialize};
use crate::common::{GenericDeviceModelStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitoringReportConfirmation {
    pub status: GenericDeviceModelStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
