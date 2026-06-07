//! 充电点状态类型

use serde::{Deserialize, Serialize};

/// 充电点/连接器的运行状态
///
/// 常见状态包括 Available、Charging、Unavailable 等，用于上报设备当前的可用性与工作阶段。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ChargePointStatus {
    Available,
    Preparing,
    Charging,
    SuspendedEvse,
    SuspendedEv,
    Finishing,
    Reserved,
    Unavailable,
    Faulted,
}

/// 连接器错误码枚举
///
/// 列举了 OCPP 中常见的错误类型，例如 OverCurrentFailure、InternalError 等。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum ChargePointErrorCode {
    #[default]
    NoError,
    ConnectorLockFailure,
    EvCommunicationFailure,
    GroundFailure,
    HighTemperature,
    InternalError,
    LocalListConflict,
    Mode3Error,
    OtherError,
    OverCurrentFailure,
    OverVoltage,
    PowerMeterFailure,
    PowerSwitchFailure,
    ReaderFailure,
    ResetFailure,
    UnderVoltage,
    WeakSignal,
}

/// 更改可用性请求的返回状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum AvailabilityStatus {
    Accepted,
    Rejected,
    Scheduled,
}

/// 可用性类型（用于 ChangeAvailability 请求），表示目标可用性
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum AvailabilityType {
    Operative,
    Suspended,
    Inoperative,
}

/// BootNotification 响应中的注册状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum RegistrationStatus {
    Accepted,
    Pending,
    Rejected,
}

/// 远程启动/停止操作的返回状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum RemoteStartStopStatus {
    Accepted,
    Rejected,
}

/// DataTransfer 操作的返回状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum DataTransferStatus {
    Accepted,
    Rejected,
    UnknownVendorId,
}

/// 诊断（Diagnostics）流程的状态枚举
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum DiagnosticsStatus {
    Idle,
    Downloading,
    Downloaded,
    Installing,
    Installed,
    UploadFailed,
}

/// 固件更新（Firmware）状态枚举
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum FirmwareStatus {
    Idle,
    Downloading,
    Downloaded,
    Installing,
    Installed,
    DownloadFailed,
    InstallationFailed,
}

/// ReserveNow / 取消预订 等操作的状态返回枚举
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ReservationStatus {
    Accepted,
    Faulted,
    Occupied,
    Rejected,
    Unavailable,
}

/// 取消预订的返回状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum CancelReservationStatus {
    Accepted,
    Rejected,
}

/// 固件或配置更新的返回状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum UpdateStatus {
    Accepted,
    Failed,
    NotSupported,
    VersionMismatch,
}

/// 更新类型（差分/完整）
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum UpdateType {
    Differential,
    Full,
}

/// 设置充电档案操作的返回状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingProfileStatus {
    Accepted,
    Rejected,
    NotSupported,
}

/// 清除充电档案的返回状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ClearChargingProfileStatus {
    Accepted,
    Unknown,
}

/// GetCompositeSchedule 操作的返回状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum GetCompositeScheduleStatus {
    Accepted,
    Rejected,
}

/// TriggerMessage 操作的返回状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerMessageStatus {
    Accepted,
    Rejected,
    NotImplemented,
}

/// UnlockConnector 操作的返回状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum UnlockStatus {
    Unlocked,
    UnlockFailed,
    NotSupported,
}

/// ClearCache 操作的返回状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ClearCacheStatus {
    Accepted,
    Rejected,
}

/// ChangeConfiguration / SetConfiguration 操作的返回状态
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ConfigurationStatus {
    Accepted,
    Rejected,
    RebootRequired,
}

/// 可被触发的消息类型列表（用于 TriggerMessage 请求）
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum MessageTrigger {
    BootNotification,
    DiagnosticsStatusNotification,
    FirmwareStatusNotification,
    Heartbeat,
    MeterValues,
    StatusNotification,
}

/// 充电速率单位（W 或 A）
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingRateUnit {
    W,
    A,
}

/// 充电档案用途枚举（例如用于 tx 或整个充电点）
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingProfilePurpose {
    ChargePointMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

/// 充电档案的类型（绝对 / 相对 / 周期性）
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingProfileKind {
    Absolute,
    Relative,
    Recurring,
}

/// 重复性类型（Daily / Weekly）
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum RecurrencyKind {
    Daily,
    Weekly,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ChargePointStatus 测试
    #[test]
    fn test_charge_point_status_available() {
        let status = ChargePointStatus::Available;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"Available\"");
        let de: ChargePointStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, de);
    }

    #[test]
    fn test_charge_point_status_charging() {
        let status = ChargePointStatus::Charging;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"Charging\"");
        let de: ChargePointStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, de);
    }

    #[test]
    fn test_charge_point_status_all_variants() {
        let variants = [
            ChargePointStatus::Available,
            ChargePointStatus::Preparing,
            ChargePointStatus::Charging,
            ChargePointStatus::SuspendedEvse,
            ChargePointStatus::SuspendedEv,
            ChargePointStatus::Finishing,
            ChargePointStatus::Reserved,
            ChargePointStatus::Unavailable,
            ChargePointStatus::Faulted,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ChargePointStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    // ChargePointErrorCode 测试
    #[test]
    fn test_charge_point_error_code_serialization() {
        let code = ChargePointErrorCode::NoError;
        let json = serde_json::to_string(&code).unwrap();
        assert_eq!(json, "\"NoError\"");
    }

    #[test]
    fn test_charge_point_error_code_deserialization() {
        let de: ChargePointErrorCode = serde_json::from_str(r#""InternalError""#).unwrap();
        assert_eq!(de, ChargePointErrorCode::InternalError);
    }

    #[test]
    fn test_charge_point_error_code_default() {
        let code = ChargePointErrorCode::default();
        assert_eq!(code, ChargePointErrorCode::NoError);
    }

    #[test]
    fn test_charge_point_error_code_all_variants() {
        let variants = [
            ChargePointErrorCode::NoError,
            ChargePointErrorCode::ConnectorLockFailure,
            ChargePointErrorCode::EvCommunicationFailure,
            ChargePointErrorCode::GroundFailure,
            ChargePointErrorCode::HighTemperature,
            ChargePointErrorCode::InternalError,
            ChargePointErrorCode::LocalListConflict,
            ChargePointErrorCode::Mode3Error,
            ChargePointErrorCode::OtherError,
            ChargePointErrorCode::OverCurrentFailure,
            ChargePointErrorCode::OverVoltage,
            ChargePointErrorCode::PowerMeterFailure,
            ChargePointErrorCode::PowerSwitchFailure,
            ChargePointErrorCode::ReaderFailure,
            ChargePointErrorCode::ResetFailure,
            ChargePointErrorCode::UnderVoltage,
            ChargePointErrorCode::WeakSignal,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ChargePointErrorCode = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    // RegistrationStatus 测试
    #[test]
    fn test_registration_status_serialization() {
        assert_eq!(
            serde_json::to_string(&RegistrationStatus::Accepted).unwrap(),
            "\"Accepted\""
        );
        assert_eq!(
            serde_json::to_string(&RegistrationStatus::Pending).unwrap(),
            "\"Pending\""
        );
        assert_eq!(
            serde_json::to_string(&RegistrationStatus::Rejected).unwrap(),
            "\"Rejected\""
        );
    }

    #[test]
    fn test_registration_status_deserialization() {
        assert_eq!(
            serde_json::from_str::<RegistrationStatus>(r#""Accepted""#).unwrap(),
            RegistrationStatus::Accepted
        );
        assert_eq!(
            serde_json::from_str::<RegistrationStatus>(r#""Pending""#).unwrap(),
            RegistrationStatus::Pending
        );
        assert_eq!(
            serde_json::from_str::<RegistrationStatus>(r#""Rejected""#).unwrap(),
            RegistrationStatus::Rejected
        );
    }

    // RemoteStartStopStatus 测试
    #[test]
    fn test_remote_start_stop_status() {
        let status = RemoteStartStopStatus::Accepted;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"Accepted\"");
        let de: RemoteStartStopStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, de);
    }

    // DataTransferStatus 测试
    #[test]
    fn test_data_transfer_status() {
        let status = DataTransferStatus::UnknownVendorId;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"UnknownVendorId\"");
        let de: DataTransferStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, de);
    }

    // DiagnosticsStatus 测试
    #[test]
    fn test_diagnostics_status_all_variants() {
        let variants = [
            DiagnosticsStatus::Idle,
            DiagnosticsStatus::Downloading,
            DiagnosticsStatus::Downloaded,
            DiagnosticsStatus::Installing,
            DiagnosticsStatus::Installed,
            DiagnosticsStatus::UploadFailed,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: DiagnosticsStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    // FirmwareStatus 测试
    #[test]
    fn test_firmware_status_all_variants() {
        let variants = [
            FirmwareStatus::Idle,
            FirmwareStatus::Downloading,
            FirmwareStatus::Downloaded,
            FirmwareStatus::Installing,
            FirmwareStatus::Installed,
            FirmwareStatus::DownloadFailed,
            FirmwareStatus::InstallationFailed,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: FirmwareStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    // ReservationStatus 测试
    #[test]
    fn test_reservation_status_all_variants() {
        let variants = [
            ReservationStatus::Accepted,
            ReservationStatus::Faulted,
            ReservationStatus::Occupied,
            ReservationStatus::Rejected,
            ReservationStatus::Unavailable,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ReservationStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    // MessageTrigger 测试
    #[test]
    fn test_message_trigger_all_variants() {
        let variants = [
            MessageTrigger::BootNotification,
            MessageTrigger::DiagnosticsStatusNotification,
            MessageTrigger::FirmwareStatusNotification,
            MessageTrigger::Heartbeat,
            MessageTrigger::MeterValues,
            MessageTrigger::StatusNotification,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: MessageTrigger = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    // ChargingRateUnit 测试
    #[test]
    fn test_charging_rate_unit() {
        assert_eq!(
            serde_json::to_string(&ChargingRateUnit::W).unwrap(),
            "\"W\""
        );
        assert_eq!(
            serde_json::to_string(&ChargingRateUnit::A).unwrap(),
            "\"A\""
        );
    }

    // ChargingProfilePurpose 测试
    #[test]
    fn test_charging_profile_purpose_all_variants() {
        let variants = [
            ChargingProfilePurpose::ChargePointMaxProfile,
            ChargingProfilePurpose::TxDefaultProfile,
            ChargingProfilePurpose::TxProfile,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ChargingProfilePurpose = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    // ChargingProfileKind 测试
    #[test]
    fn test_charging_profile_kind_all_variants() {
        let variants = [
            ChargingProfileKind::Absolute,
            ChargingProfileKind::Relative,
            ChargingProfileKind::Recurring,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ChargingProfileKind = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    // RecurrencyKind 测试
    #[test]
    fn test_recurrency_kind_all_variants() {
        let variants = [RecurrencyKind::Daily, RecurrencyKind::Weekly];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: RecurrencyKind = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }
}
