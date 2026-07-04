//! Response Status Enums for OCPP 2.0.1
//! OCPP 2.0.1 响应状态枚举集合

use serde::{Deserialize, Serialize};

// =========== Reservation (Functional Block H) ===========

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

// =========== Local Auth (Functional Block C) ===========

/// SendLocalList 响应状态（已在 authorization.rs）

/// GetLocalListVersion 响应

// =========== Smart Charging (Functional Block A) ===========

/// 设置充电曲线响应状态 (已在 charging_profile.rs 定义为 ChargingProfileStatusEnumType)

/// 清除充电曲线响应状态 (已在 charging_profile.rs 定义为 ClearChargingProfileStatusEnumType)

/// GetChargingProfiles 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GetChargingProfilesStatusEnumType {
    Accepted,
    NoProfiles,
}

/// GetCompositeSchedule 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GenericStatusEnumType {
    Accepted,
    Rejected,
}

// =========== Firmware Management (Functional Block I) ===========

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

/// PublishFirmware 响应状态
/// 注：GenericDeviceModelStatusEnumType 在 charging_profile.rs 中定义（被多个响应共享）

/// UnpublishFirmware 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UnpublishFirmwareStatusEnumType {
    DownloadOngoing,
    NoFirmware,
    Unpublished,
}

// =========== Monitoring (Functional Block D) ===========

/// SetVariableMonitoring 响应
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SetMonitoringStatusEnumType {
    Accepted,
    UnknownComponent,
    UnknownVariable,
    UnsupportedMonitorType,
    Rejected,
    DuplicateValue,
}

/// ClearVariableMonitoring 响应
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClearMonitoringStatusEnumType {
    Accepted,
    Rejected,
    NotFound,
}

/// SetMonitoringBase 响应 (与 ClearVariableMonitoring 状态复用)

/// SetMonitoringLevel 响应 (与 ClearVariableMonitoring 状态复用)

// =========== Certificate Management (Functional Block J) ===========

/// SignCertificate 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GenericStatusEnumType2 {
    Accepted,
    Rejected,
}

/// 重新命名避免冲突：SignCertificate 与 Generic
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

// =========== Display Management (Functional Block L) ===========

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

// =========== Customer Information (Functional Block G) ===========

/// CustomerInformation 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CustomerInformationStatusEnumType {
    Accepted,
    Rejected,
    Invalid,
}

// =========== ISO 15118 (Functional Block B) ===========

/// NotifyEVChargingNeeds 响应状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum NotifyEVChargingNeedsStatusEnumType {
    Accepted,
    Rejected,
    Processing,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_roundtrip_enum {
        ($name:ident, $enum_ty:ident, $variant1:ident, $variant2:ident) => {
            #[test]
            fn $name() {
                let variants = [$enum_ty::$variant1, $enum_ty::$variant2];
                for v in variants {
                    let json = serde_json::to_string(&v).unwrap();
                    let de: $enum_ty = serde_json::from_str(&json).unwrap();
                    assert_eq!(v, de);
                }
            }
        };
    }

    test_roundtrip_enum!(
        test_reserve_now_status,
        ReserveNowStatusEnumType,
        Accepted,
        Rejected
    );
    test_roundtrip_enum!(
        test_cancel_reservation_status,
        CancelReservationStatusEnumType,
        Accepted,
        Rejected
    );
    test_roundtrip_enum!(
        test_get_charging_profiles_status,
        GetChargingProfilesStatusEnumType,
        Accepted,
        NoProfiles
    );
    test_roundtrip_enum!(
        test_generic_status,
        GenericStatusEnumType,
        Accepted,
        Rejected
    );
    test_roundtrip_enum!(
        test_sign_certificate_status,
        SignCertificateStatusEnumType,
        Accepted,
        Rejected
    );
    test_roundtrip_enum!(
        test_certificate_signed_status,
        CertificateSignedStatusEnumType,
        Accepted,
        Rejected
    );
    test_roundtrip_enum!(
        test_install_certificate_status,
        InstallCertificateStatusEnumType,
        Accepted,
        Rejected
    );
    test_roundtrip_enum!(
        test_delete_certificate_status,
        DeleteCertificateStatusEnumType,
        Accepted,
        NotFound
    );
    test_roundtrip_enum!(
        test_get_installed_certificate_status,
        GetInstalledCertificateStatusEnumType,
        Accepted,
        NotFound
    );
    test_roundtrip_enum!(
        test_display_message_status,
        DisplayMessageStatusEnumType,
        Accepted,
        Rejected
    );
    test_roundtrip_enum!(
        test_clear_message_status,
        ClearMessageStatusEnumType,
        Accepted,
        Unknown
    );
    test_roundtrip_enum!(
        test_customer_information_status,
        CustomerInformationStatusEnumType,
        Accepted,
        Rejected
    );
    test_roundtrip_enum!(
        test_notify_ev_charging_needs_status,
        NotifyEVChargingNeedsStatusEnumType,
        Accepted,
        Rejected
    );
}
