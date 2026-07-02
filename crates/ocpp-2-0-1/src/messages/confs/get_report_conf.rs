//! GetReport Confirmation (与 GetBaseReport 相同的状态类型)

use serde::{Deserialize, Serialize};
use crate::common::{GenericDeviceModelStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReportConfirmation {
    pub status: GenericDeviceModelStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl GetReportConfirmation {
    pub fn new(status: GenericDeviceModelStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
        }
    }

    pub fn accepted() -> Self {
        Self::new(GenericDeviceModelStatusEnumType::Accepted)
    }
}