//! CancelReservation Confirmation (Block H)
use serde::{Deserialize, Serialize};
use crate::common::{CancelReservationStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationConfirmation {
    pub status: CancelReservationStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
