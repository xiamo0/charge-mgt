//! Status and Availability Types (Functional Block B / F / G / H)

use serde::{Deserialize, Serialize};

/// 连接器状态枚举 (StatusNotification)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ConnectorStatusEnumType {
    Available,
    Occupied,
    Reserved,
    Unavailable,
    Faulted,
}

/// 运营状态枚举 (ChangeAvailability)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OperationalStatusEnumType {
    Operative,
    Inoperative,
}

/// 变更可用性状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChangeAvailabilityStatusEnumType {
    Accepted,
    Rejected,
    Scheduled,
}

/// 注册状态枚举 (BootNotification)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RegistrationStatusEnumType {
    Accepted,
    Rejected,
    Pending,
}

/// RequestStartStop 状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RequestStartStopStatusEnumType {
    Accepted,
    Rejected,
}

/// 重启状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ResetStatusEnumType {
    Accepted,
    Rejected,
    Scheduled,
}

/// 重启类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ResetEnumType {
    Immediate,
    OnIdle,
}

/// 解锁状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UnlockStatusEnumType {
    Unlocked,
    UnlockFailed,
    OngoingAuthorizedTransaction,
}

/// 清除缓存状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClearCacheStatusEnumType {
    Accepted,
    Rejected,
}

/// 触发消息状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerMessageStatusEnumType {
    Accepted,
    Rejected,
    NotImplemented,
}

/// 触发消息类型枚举 (TriggerMessage.req)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MessageTriggerEnumType {
    BootNotification,
    LogStatusNotification,
    FirmwareStatusNotification,
    Heartbeat,
    MeterValues,
    SignChargingStationCertificate,
    SignV2GCertificate,
    StatusNotification,
    TransactionEvent,
    SignCombinedCertificate,
    PublishFirmwareStatusNotification,
}

/// 预约状态更新枚举 (ReservationStatusUpdate.req)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ReservationUpdateStatusEnumType {
    Expired,
    Removed,
}
