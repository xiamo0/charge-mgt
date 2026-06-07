//! OCPP 1.6 协议常量
//!
//! 此模块汇总了协议版本字符串、消息类型 ID、常用操作名称以及标准错误码，供其他模块复用。

/// 协议名称（包含版本）
pub const PROTOCOL_VERSION: &str = "OCPP-1.6";
/// OCPP 规范版本号（仅数字）
pub const OCPP_VERSION: &str = "1.6";

// 消息类型 ID（用于消息首位标识）
/// CALL 消息类型 ID（请求）
pub const CALL: i32 = 2;
/// CALLRESULT 消息类型 ID（成功响应）
pub const CALLRESULT: i32 = 3;
/// CALLERROR 消息类型 ID（错误响应）
pub const CALLERROR: i32 = 4;

// 操作名称：常用的 OCPP 操作字符串常量，便于避免魔法字符串
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

// 错误码（标准字符串），用于构造 CallError 中的 errorCode 字段
pub mod errors {
    /// 操作未实现
    pub const NOT_IMPLEMENTED: &str = "NotImplemented";
    /// 操作不支持
    pub const NOT_SUPPORTED: &str = "NotSupported";
    /// 内部错误
    pub const INTERNAL_ERROR: &str = "InternalError";
    /// 协议错误
    pub const PROTOCOL_ERROR: &str = "ProtocolError";
    /// 安全相关错误
    pub const SECURITY_ERROR: &str = "SecurityError";
    /// 消息形成/格式错误
    pub const FORMATION_VIOLATION: &str = "FormationViolation";
    /// 属性约束违规
    pub const PROPERTY_CONSTRAINT_VIOLATION: &str = "PropertyConstraintViolation";
    /// 出现次数约束违规
    pub const OCCURENCE_CONSTRAINT_VIOLATION: &str = "OccurenceConstraintViolation";
    /// 类型约束违规
    pub const TYPE_CONSTRAINT_VIOLATION: &str = "TypeConstraintViolation";
    /// 通用错误
    pub const GENERIC_ERROR: &str = "GenericError";
}
