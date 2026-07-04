//! 业务枚举集中管理。
//!
//! 把数据库列上的枚举值（状态、类型标识）用 Rust enum 表达，避免裸 `i16` / `String`
//! 散落在业务层。本模块按 **DB 列数据类型** 分两类：
//!
//! * **`SMALLINT` 列**：使用 [`impl_i16_enum!`] 宏一次性展开 `sea-orm` 所需的全套 trait
//!   （`From<T> for Value` / `Nullable` / `ValueType` / `TryGetable`），
//!   并配合 [`serde_repr`] 让 JSON 序列化输出仍为整数。
//! * **`VARCHAR` 列**：保持 `String` 字段存储，枚举仅用于业务层的类型化映射
//!   （[`ChargePointStatus`] / [`ChargingProfilePurpose`] / [`ChargingProfileKind`]），
//!   通过 `as_str` / `from_str_opt` 与字符串互转。
//!
//! # 枚举一览
//!
//! | 枚举                        | DB 类型  | 取值来源                            |
//! | --------------------------- | -------- | ----------------------------------- |
//! | [`ConnectorType`]           | SMALLINT | 充电枪接口（国标直流/交流）         |
//! | [`TagType`]                 | SMALLINT | 身份标签种类                        |
//! | [`IdentityStatus`]          | SMALLINT | 标签鉴权状态（有效/挂失/过期）      |
//! | [`TransactionStatus`]       | SMALLINT | 事务阶段                            |
//! | [`PaymentStatus`]           | SMALLINT | 支付状态                            |
//! | [`ReservationStatus`]       | SMALLINT | 预约生命周期                        |
//! | [`ProfileDeliveryStatus`]   | SMALLINT | OCPP 策略下发结果                   |
//! | [`ChargePointStatus`]       | VARCHAR  | OCPP ChargePointStatus 协议枚举     |
//! | [`ChargingProfilePurpose`]  | VARCHAR  | OCPP SetChargingProfile purpose     |
//! | [`ChargingProfileKind`]     | VARCHAR  | OCPP SetChargingProfile kind        |
//!
//! # 为何不使用 `DeriveValueType`
//!
//! sea-orm 2.0-rc 的 `DeriveValueType` derive 宏在当前版本下未能可靠生成 `ValueType`
//! 实现（编译时会缺失相关 trait 方法），因此本模块通过宏显式展开以避免依赖不稳定的
//! derive 行为。

use sea_orm::{
    DbErr, QueryResult, TryGetError, TryGetable, sea_query,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 为 i16-backed 枚举生成全套 sea-orm trait。
///
/// `DeriveValueType` derive macro in sea-orm 2.0-rc 在当前版本下未可靠生成 `ValueType`，
/// 所以本宏显式展开：`From<T> for Value` / `Nullable` / `ValueType` / `TryGetable`，
/// 以及常规的 `TryFrom<i16>` / `From<T> for i16`。
macro_rules! impl_i16_enum {
    ($t:ty, [ $( $variant:ident = $val:expr ),+ $(,)? ]) => {
        impl TryFrom<i16> for $t {
            type Error = String;
            fn try_from(v: i16) -> Result<Self, Self::Error> {
                match v {
                    $( $val => Ok(Self::$variant), )+
                    other => Err(format!("unknown {} value: {other}", stringify!($t))),
                }
            }
        }

        impl From<$t> for i16 {
            fn from(v: $t) -> i16 {
                v as i16
            }
        }

        impl From<$t> for sea_query::Value {
            fn from(v: $t) -> sea_query::Value {
                sea_query::Value::SmallInt(Some(v as i16))
            }
        }

        impl sea_query::Nullable for $t {
            fn null() -> sea_query::Value {
                sea_query::Value::SmallInt(None)
            }
        }

        impl sea_query::ValueType for $t {
            fn try_from(v: sea_query::Value) -> Result<Self, sea_query::ValueTypeErr> {
                match v {
                    sea_query::Value::SmallInt(Some(x)) => <$t as TryFrom<i16>>::try_from(x)
                        .map_err(|_| sea_query::ValueTypeErr),
                    _ => Err(sea_query::ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($t).to_owned()
            }

            fn array_type() -> sea_query::ArrayType {
                sea_query::ArrayType::SmallInt
            }

            fn column_type() -> sea_query::ColumnType {
                sea_query::ColumnType::SmallInteger
            }
        }

        impl TryGetable for $t {
            fn try_get_by<I: sea_orm::ColIdx>(
                res: &QueryResult,
                index: I,
            ) -> Result<Self, TryGetError> {
                let v: i16 = res.try_get_by(index).map_err(TryGetError::DbErr)?;
                <$t as TryFrom<i16>>::try_from(v)
                    .map_err(|msg| TryGetError::DbErr(DbErr::Json(msg)))
            }
        }
    };
}

// =============================================================================
// SMALLINT-backed 枚举
// =============================================================================

/// 连接器/充电枪接口类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i16)]
pub enum ConnectorType {
    /// 国标直流
    GbtDc = 1,
    /// 国标交流
    GbtAc = 2,
}

/// 用户身份标签类型。
///
/// 对应 `identity_info.tag_type` 列，标识触发鉴权/启动充电的身份载体。
/// `AppToken` 用于 App 一键启动等无需实体介质的场景。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i16)]
pub enum TagType {
    /// RFID 实体卡
    Rfid = 1,
    /// App / Web 端扫码二维码
    QrCode = 2,
    /// 车牌识别
    LicensePlate = 3,
    /// App 内部生成的虚拟 Token（如一键充电）
    AppToken = 4,
}

/// 身份标签鉴权状态。
///
/// 对应 `identity_info.status` 列，Authorize 时直接用于判定是否放行；
/// 状态变更通过 [`identity::to_blocked_status`](crate::service::identity::to_blocked_status)
/// 或 [`identity::activate`](crate::service::identity::activate) 触发。
///
/// 注意：`Blocked` 是物理删除的替代——保留所有审计轨迹，仅通过该状态位屏蔽使用。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i16)]
pub enum IdentityStatus {
    /// 有效（对应 OCPP Authorize.Accepted）
    Accepted = 1,
    /// 挂失/无效（对应 OCPP Authorize.Blocked）
    Blocked = 2,
    /// 已过期（由定时任务或 `expire_time < now` 判定）
    Expired = 3,
}

/// 充电事务状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i16)]
pub enum TransactionStatus {
    /// 进行中
    InProgress = 0,
    /// 正常结束
    Completed = 1,
    /// 异常中断
    Aborted = 2,
    /// 离线补传待处理
    OfflinePending = 3,
}

/// 支付状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i16)]
pub enum PaymentStatus {
    /// 未支付
    Unpaid = 0,
    /// 已支付
    Paid = 1,
    /// 支付失败
    Failed = 2,
    /// 已退款
    Refunded = 3,
}

/// 充电预约状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i16)]
pub enum ReservationStatus {
    /// 待履约
    Pending = 0,
    /// 进行中（已扫码）
    InProgress = 1,
    /// 已完成
    Completed = 2,
    /// 已取消
    Cancelled = 3,
    /// 已违约
    Breached = 4,
}

/// 智能充电策略下发状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i16)]
pub enum ProfileDeliveryStatus {
    /// 待下发
    Pending = 0,
    /// 已接受（Accepted）
    Accepted = 1,
    /// 已拒绝（Rejected）
    Rejected = 2,
}

impl_i16_enum!(ConnectorType, [GbtDc = 1, GbtAc = 2]);
impl_i16_enum!(TagType, [Rfid = 1, QrCode = 2, LicensePlate = 3, AppToken = 4]);
impl_i16_enum!(IdentityStatus, [Accepted = 1, Blocked = 2, Expired = 3]);
impl_i16_enum!(
    TransactionStatus,
    [InProgress = 0, Completed = 1, Aborted = 2, OfflinePending = 3]
);
impl_i16_enum!(PaymentStatus, [Unpaid = 0, Paid = 1, Failed = 2, Refunded = 3]);
impl_i16_enum!(
    ReservationStatus,
    [Pending = 0, InProgress = 1, Completed = 2, Cancelled = 3, Breached = 4]
);
impl_i16_enum!(
    ProfileDeliveryStatus,
    [Pending = 0, Accepted = 1, Rejected = 2]
);

// =============================================================================
// VARCHAR-backed 业务枚举（仅用于业务层，DB 列仍为 String）
// =============================================================================

/// OCPP ChargePointStatus 枚举（用于 `charge_point.status` 与
/// `charge_connector.status` 字段）。
///
/// DB 列使用 VARCHAR，直接映射 OCPP 协议字符串。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChargePointStatus {
    Available,
    Preparing,
    Charging,
    SuspendedEVSE,
    SuspendedEV,
    Finishing,
    Reserved,
    Unavailable,
    Faulted,
}

impl ChargePointStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Available => "Available",
            Self::Preparing => "Preparing",
            Self::Charging => "Charging",
            Self::SuspendedEVSE => "SuspendedEVSE",
            Self::SuspendedEV => "SuspendedEV",
            Self::Finishing => "Finishing",
            Self::Reserved => "Reserved",
            Self::Unavailable => "Unavailable",
            Self::Faulted => "Faulted",
        }
    }

    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s {
            "Available" => Some(Self::Available),
            "Preparing" => Some(Self::Preparing),
            "Charging" => Some(Self::Charging),
            "SuspendedEVSE" => Some(Self::SuspendedEVSE),
            "SuspendedEV" => Some(Self::SuspendedEV),
            "Finishing" => Some(Self::Finishing),
            "Reserved" => Some(Self::Reserved),
            "Unavailable" => Some(Self::Unavailable),
            "Faulted" => Some(Self::Faulted),
            _ => None,
        }
    }
}

/// OCPP 智能充电策略 purpose。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChargingProfilePurpose {
    ChargePointMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

impl ChargingProfilePurpose {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ChargePointMaxProfile => "ChargePointMaxProfile",
            Self::TxDefaultProfile => "TxDefaultProfile",
            Self::TxProfile => "TxProfile",
        }
    }

    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s {
            "ChargePointMaxProfile" => Some(Self::ChargePointMaxProfile),
            "TxDefaultProfile" => Some(Self::TxDefaultProfile),
            "TxProfile" => Some(Self::TxProfile),
            _ => None,
        }
    }
}

/// OCPP 智能充电策略 kind。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChargingProfileKind {
    Absolute,
    Recurring,
    Relative,
}

impl ChargingProfileKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Absolute => "Absolute",
            Self::Recurring => "Recurring",
            Self::Relative => "Relative",
        }
    }

    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s {
            "Absolute" => Some(Self::Absolute),
            "Recurring" => Some(Self::Recurring),
            "Relative" => Some(Self::Relative),
            _ => None,
        }
    }
}

impl From<ChargePointStatus> for String {
    fn from(v: ChargePointStatus) -> String {
        v.as_str().to_owned()
    }
}

impl From<ChargingProfilePurpose> for String {
    fn from(v: ChargingProfilePurpose) -> String {
        v.as_str().to_owned()
    }
}

impl From<ChargingProfileKind> for String {
    fn from(v: ChargingProfileKind) -> String {
        v.as_str().to_owned()
    }
}
