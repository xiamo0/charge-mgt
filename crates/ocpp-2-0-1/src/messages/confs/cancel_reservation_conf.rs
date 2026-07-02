//! CancelReservation Confirmation
use serde::{Deserialize, Serialize};
use crate::common::CancelReservationStatusEnumType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationConfirmation {
    pub status: CancelReservationStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<crate::common::StatusInfoType>,
}
