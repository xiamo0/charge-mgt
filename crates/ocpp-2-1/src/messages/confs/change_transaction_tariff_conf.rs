//! ChangeTransactionTariff Confirmation (Block I — 2.1)
use crate::common::{StatusInfoType, TariffChangeStatusEnumType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTransactionTariffConfirmation {
    pub status: TariffChangeStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
