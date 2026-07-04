//! SetDERControl Confirmation (Block R — 2.1 New)
use crate::common::{DERControlStatusEnumType, StatusInfoType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDERControlConfirmation {
    pub status: DERControlStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superseded_ids: Option<Vec<String>>,
}
