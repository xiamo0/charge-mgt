use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::entity::enums::{PaymentStatus, TransactionStatus};

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTransaction {
    pub status: Option<TransactionStatus>,
    pub stop_reason: Option<String>,
    pub end_time: Option<NaiveDateTime>,
    pub meter_stop: Option<Decimal>,
    pub total_energy: Option<Decimal>,
    pub payment_status: Option<PaymentStatus>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SettleTransaction {
    pub total_amount: Decimal,
    pub electricity_fee: Decimal,
    pub service_fee: Decimal,
    #[serde(default)]
    pub payment_status: Option<PaymentStatus>,
}

#[derive(Debug, Default, Deserialize)]
pub struct TransactionListQuery {
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub charge_point_id: Option<String>,
    #[serde(default)]
    pub status: Option<TransactionStatus>,
    #[serde(default)]
    pub payment_status: Option<PaymentStatus>,
    #[serde(default)]
    pub start_time_from: Option<NaiveDateTime>,
    #[serde(default)]
    pub start_time_to: Option<NaiveDateTime>,
    #[serde(default)]
    pub include_offline_sync: Option<bool>,
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub page_size: Option<u64>,
}

impl TransactionListQuery {
    pub fn page_query(&self) -> super::common::PageQuery {
        super::common::PageQuery {
            page: self.page.unwrap_or(1),
            page_size: self.page_size.unwrap_or(20),
        }
        .normalize()
    }
}

pub type TransactionResponse = crate::entity::charge_transaction::Model;
