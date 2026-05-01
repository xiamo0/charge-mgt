//! 充电点状态类型

use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum AvailabilityStatus {
    Accepted,
    Rejected,
    Scheduled,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum AvailabilityType {
    Operative,
    Suspended,
    Inoperative,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum RegistrationStatus {
    Accepted,
    Pending,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum RemoteStartStopStatus {
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum DataTransferStatus {
    Accepted,
    Rejected,
    UnknownVendorId,
}

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ReservationStatus {
    Accepted,
    Faulted,
    Occupied,
    Rejected,
    Unavailable,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum CancelReservationStatus {
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum UpdateStatus {
    Accepted,
    Failed,
    NotSupported,
    VersionMismatch,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum UpdateType {
    Differential,
    Full,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingProfileStatus {
    Accepted,
    Rejected,
    NotSupported,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ClearChargingProfileStatus {
    Accepted,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum GetCompositeScheduleStatus {
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerMessageStatus {
    Accepted,
    Rejected,
    NotImplemented,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum UnlockStatus {
    Unlocked,
    UnlockFailed,
    NotSupported,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ClearCacheStatus {
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ConfigurationStatus {
    Accepted,
    Rejected,
    RebootRequired,
}

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingRateUnit {
    W,
    A,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingProfilePurpose {
    ChargePointMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingProfileKind {
    Absolute,
    Relative,
    Recurring,
}

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
