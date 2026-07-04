//! SetDefaultTariff Confirmation (Block I — 2.1)
use serde::{Deserialize, Serialize};
use crate::common::{TariffSetStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultTariffConfirmation {
    pub status: TariffSetStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
