//! NotifySettlement Request (Block I — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::{AddressType, PaymentStatusEnumType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifySettlementRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    pub psp_ref: String,
    pub settlement_amount: f64,
    pub settlement_time: String,
    pub status: PaymentStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_company: Option<AddressType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<String>,
}

pub const ACTION: &str = "NotifySettlement";
