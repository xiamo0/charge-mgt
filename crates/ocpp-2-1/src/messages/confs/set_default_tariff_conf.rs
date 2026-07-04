//! SetDefaultTariff Confirmation (Block I — 2.1)
use crate::common::{StatusInfoType, TariffSetStatusEnumType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultTariffConfirmation {
    pub status: TariffSetStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
