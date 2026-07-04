//! OCPP 2.0.1 Response Status Enums
//! 保留自 OCPP 2.0.1 的响应状态枚举（2.1 新增状态在各自领域模块中）

use serde::{Deserialize, Serialize};

// =========== Block H — Reservation ============

/// ReserveNow 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ReserveNowStatusEnumType {
    Accepted,
    Faulted,
    Occupied,
    Rejected,
    Unavailable,
}

/// CancelReservation 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CancelReservationStatusEnumType {
    Accepted,
    Rejected,
}

// =========== Block K — SmartCharging ============

/// GetChargingProfiles 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GetChargingProfilesStatusEnumType {
    Accepted,
    NoProfiles,
}

/// 通用状态枚举 (多个消息复用)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GenericStatusEnumType {
    Accepted,
    Rejected,
}

// =========== Block L — Firmware ============

/// UpdateFirmware 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UpdateFirmwareStatusEnumType {
    Accepted,
    Rejected,
    AcceptedCanceled,
    InvalidCertificate,
    RevokedCertificate,
}

/// GetLog 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LogStatusEnumType {
    Accepted,
    Rejected,
    AcceptedCanceled,
}

/// UnpublishFirmware 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UnpublishFirmwareStatusEnumType {
    DownloadOngoing,
    NoFirmware,
    Unpublished,
}

// =========== Block M — Certificates ============

/// SignCertificate 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SignCertificateStatusEnumType {
    Accepted,
    Rejected,
}

/// CertificateSigned 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CertificateSignedStatusEnumType {
    Accepted,
    Rejected,
}

/// InstallCertificate 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum InstallCertificateStatusEnumType {
    Accepted,
    Rejected,
    Failed,
}

/// DeleteCertificate 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DeleteCertificateStatusEnumType {
    Accepted,
    Failed,
    NotFound,
}

/// GetInstalledCertificateIds 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GetInstalledCertificateStatusEnumType {
    Accepted,
    NotFound,
}

/// GetCertificateStatus 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GetCertificateStatusEnumType {
    Accepted,
    Failed,
}

/// Get15118EVCertificate 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Iso15118EVCertificateStatusEnumType {
    Accepted,
    Failed,
}

// =========== Block O — Display ============

/// SetDisplayMessage 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DisplayMessageStatusEnumType {
    Accepted,
    NotSupportedMessageFormat,
    Rejected,
    NotSupportedPriority,
    NotSupportedState,
    UnknownTransaction,
}

/// ClearDisplayMessage 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClearMessageStatusEnumType {
    Accepted,
    Unknown,
}

/// GetDisplayMessages 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GetDisplayMessagesStatusEnumType {
    Accepted,
    Unknown,
}

// =========== Block N — Diagnostics ============

/// CustomerInformation 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CustomerInformationStatusEnumType {
    Accepted,
    Rejected,
    Invalid,
}

/// NotifyEVChargingNeeds 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum NotifyEVChargingNeedsStatusEnumType {
    Accepted,
    Rejected,
    Processing,
}

