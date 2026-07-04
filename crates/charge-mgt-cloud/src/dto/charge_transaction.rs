//! 充电事务资源 DTO。
//!
//! 事务由 OCPP StartTransaction 创建，HTTP 入口**不**支持创建。

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::entity::enums::{PaymentStatus, TransactionStatus};

/// `PATCH /api/v1/transactions/:id` 请求体。
///
/// 主要用于回写 `payment_status` 与结算金额。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTransaction {
    pub status: Option<TransactionStatus>,
    pub stop_reason: Option<String>,
    pub end_time: Option<NaiveDateTime>,
    pub meter_stop: Option<Decimal>,
    pub total_energy: Option<Decimal>,
    pub payment_status: Option<PaymentStatus>,
}

/// `POST /api/v1/transactions/:id/settle` 请求体。
///
/// `payment_status` 不提供则默认 [`PaymentStatus::Unpaid`]，便于先结算
/// 后等待第三方支付回调。
#[derive(Debug, Clone, Deserialize)]
pub struct SettleTransaction {
    pub total_amount: Decimal,
    pub electricity_fee: Decimal,
    pub service_fee: Decimal,
    #[serde(default)]
    pub payment_status: Option<PaymentStatus>,
}

/// `GET /api/v1/transactions` query string。
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
    /// 起始时间（含）
    #[serde(default)]
    pub start_time_from: Option<NaiveDateTime>,
    /// 终止时间（含）
    #[serde(default)]
    pub start_time_to: Option<NaiveDateTime>,
    /// 是否包含离线补传数据；默认 `false`（隐藏）
    #[serde(default)]
    pub include_offline_sync: Option<bool>,
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub page_size: Option<u64>,
}

impl TransactionListQuery {
    /// 转 [`super::common::PageQuery`]。
    pub fn page_query(&self) -> super::common::PageQuery {
        super::common::PageQuery {
            page: self.page.unwrap_or(1),
            page_size: self.page_size.unwrap_or(20),
        }
        .normalize()
    }
}

/// 充电事务响应体。
pub type TransactionResponse = crate::entity::charge_transaction::Model;
