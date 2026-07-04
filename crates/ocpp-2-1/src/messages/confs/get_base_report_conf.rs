//! GetBaseReport Confirmation (Block B)
use serde::{Deserialize, Serialize};
use crate::common::{GenericDeviceModelStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportConfirmation {
    pub status: GenericDeviceModelStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
