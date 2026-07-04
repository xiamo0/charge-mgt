//! OpenPeriodicEventStream Confirmation (Block N — 2.1)
use serde::{Deserialize, Serialize};
use crate::common::{GenericStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenPeriodicEventStreamConfirmation {
    pub status: GenericStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
