//! OCPP 1.6 协议常量

pub const PROTOCOL_VERSION: &str = "OCPP-1.6";
pub const OCPP_VERSION: &str = "1.6";

// 消息类型 ID
pub const CALL: i32 = 2;
pub const CALLRESULT: i32 = 3;
pub const CALLERROR: i32 = 4;

// 操作名称
pub const ACTION_AUTHORIZE: &str = "Authorize";
pub const ACTION_BOOT_NOTIFICATION: &str = "BootNotification";
pub const ACTION_HEARTBEAT: &str = "Heartbeat";
pub const ACTION_START_TRANSACTION: &str = "StartTransaction";
pub const ACTION_STOP_TRANSACTION: &str = "StopTransaction";
pub const ACTION_METER_VALUES: &str = "MeterValues";
pub const ACTION_STATUS_NOTIFICATION: &str = "StatusNotification";
pub const ACTION_REMOTE_START_TRANSACTION: &str = "RemoteStartTransaction";
pub const ACTION_REMOTE_STOP_TRANSACTION: &str = "RemoteStopTransaction";
pub const ACTION_CHANGE_AVAILABILITY: &str = "ChangeAvailability";
pub const ACTION_CHANGE_CONFIGURATION: &str = "ChangeConfiguration";
pub const ACTION_GET_CONFIGURATION: &str = "GetConfiguration";
pub const ACTION_CLEAR_CACHE: &str = "ClearCache";
pub const ACTION_UNLOCK_CONNECTOR: &str = "UnlockConnector";
pub const ACTION_DATA_TRANSFER: &str = "DataTransfer";
pub const ACTION_GET_DIAGNOSTICS: &str = "GetDiagnostics";
pub const ACTION_UPDATE_FIRMWARE: &str = "UpdateFirmware";
pub const ACTION_DIAGNOSTICS_STATUS_NOTIFICATION: &str = "DiagnosticsStatusNotification";
pub const ACTION_FIRMWARE_STATUS_NOTIFICATION: &str = "FirmwareStatusNotification";
pub const ACTION_RESERVE_NOW: &str = "ReserveNow";
pub const ACTION_CANCEL_RESERVATION: &str = "CancelReservation";
pub const ACTION_SEND_LOCAL_LIST: &str = "SendLocalList";
pub const ACTION_GET_LOCAL_LIST_VERSION: &str = "GetLocalListVersion";
pub const ACTION_SET_CHARGING_PROFILE: &str = "SetChargingProfile";
pub const ACTION_CLEAR_CHARGING_PROFILE: &str = "ClearChargingProfile";
pub const ACTION_GET_COMPOSITE_SCHEDULE: &str = "GetCompositeSchedule";
pub const ACTION_TRIGGER_MESSAGE: &str = "TriggerMessage";

// 错误码
pub mod errors {
    pub const NOT_IMPLEMENTED: &str = "NotImplemented";
    pub const NOT_SUPPORTED: &str = "NotSupported";
    pub const INTERNAL_ERROR: &str = "InternalError";
    pub const PROTOCOL_ERROR: &str = "ProtocolError";
    pub const SECURITY_ERROR: &str = "SecurityError";
    pub const FORMATION_VIOLATION: &str = "FormationViolation";
    pub const PROPERTY_CONSTRAINT_VIOLATION: &str = "PropertyConstraintViolation";
    pub const OCCURENCE_CONSTRAINT_VIOLATION: &str = "OccurenceConstraintViolation";
    pub const TYPE_CONSTRAINT_VIOLATION: &str = "TypeConstraintViolation";
    pub const GENERIC_ERROR: &str = "GenericError";
}