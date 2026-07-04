//! GetTariffs Confirmation (Block I — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::{StatusInfoType, TariffAssignmentType, TariffGetStatusEnumType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTariffsConfirmation {
    pub status: TariffGetStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff_assignments: Option<Vec<TariffAssignmentType>>,
}
