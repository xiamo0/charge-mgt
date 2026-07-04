//! OCPP 2.1 Tariff & Cost types (Functional Block I)

use serde::{Deserialize, Serialize};

/// 获取电价状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TariffGetStatusEnumType {
    Accepted,
    Rejected,
    NoTariff,
}

/// 电价种类枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TariffKindEnumType {
    DefaultTariff,
    DriverTariff,
}

/// 电价分配类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TariffAssignmentType {
    pub tariff_id: String,
    pub tariff_kind: TariffKindEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_ids: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tokens: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
}

/// 设置电价状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TariffSetStatusEnumType {
    Accepted,
    Rejected,
    TooManyElements,
    ConditionNotSupported,
    DuplicateTariffId,
}

/// 变更事务电价状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TariffChangeStatusEnumType {
    Accepted,
    Rejected,
    TooManyElements,
    ConditionNotSupported,
    TxNotFound,
    NoCurrencyChange,
}

/// 清除电价状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TariffClearStatusEnumType {
    Accepted,
    Rejected,
    NoTariff,
}

/// 清除电价结果类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsResultType {
    pub status: TariffClearStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<crate::common::StatusInfoType>,
}

/// 电价类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TariffType {
    pub tariff_id: String,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 支付状态枚举 (NotifySettlement)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PaymentStatusEnumType {
    Settled,
    Canceled,
    Rejected,
    Failed,
}
