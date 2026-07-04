//! ChangeTransactionTariff Confirmation (Block I — 2.1)
use serde::{Deserialize, Serialize};
use crate::common::{TariffChangeStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTransactionTariffConfirmation {
    pub status: TariffChangeStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
