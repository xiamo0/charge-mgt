//! RequestStartTransaction Confirmation (Block F)
use serde::{Deserialize, Serialize};
use crate::common::{RequestStartStopStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestStartTransactionConfirmation {
    pub status: RequestStartStopStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
