//! Transaction Types (Functional Block E)

use serde::{Deserialize, Serialize};

/// 事务事件类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TransactionEventEnumType {
    Started,
    Updated,
    Ended,
}

/// 触发原因枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerReasonEnumType {
    Authorized,
    CablePluggedIn,
    ChargingRateChanged,
    ChargingStateChanged,
    Deauthorized,
    EnergyLimitReached,
    EVCommunicationLost,
    EVConnectTimeout,
    MeterValueClock,
    MeterValuePeriodic,
    TimeLimitReached,
    Trigger,
    UnlockCommand,
    StopAuthorized,
    EVDeparted,
    EVDetected,
    RemoteStop,
    RemoteStart,
    AbnormalCondition,
    SignedDataReceived,
    ResetCommand,
}

/// 充电状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingStateEnumType {
    Charging,
    EVConnected,
    SuspendedEV,
    SuspendedEVSE,
    Idle,
}

/// 停止原因枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ReasonEnumType {
    DeAuthorized,
    EmergencyStop,
    EnergyLimitReached,
    EVDisconnected,
    GroundFault,
    ImmediateReset,
    Local,
    LocalOutOfCredit,
    MasterPass,
    Other,
    OvercurrentFault,
    PowerLoss,
    PowerQuality,
    Reboot,
    Remote,
    SOCLimitReached,
    StoppedByEV,
    TimeLimitReached,
    Timeout,
}

/// 事务信息类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionType {
    pub transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_state: Option<ChargingStateEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_spent_charging: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_reason: Option<ReasonEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_start_id: Option<i32>,
}

impl TransactionType {
    pub fn new(transaction_id: impl Into<String>) -> Self {
        Self {
            transaction_id: transaction_id.into(),
            charging_state: None,
            time_spent_charging: None,
            stopped_reason: None,
            remote_start_id: None,
        }
    }

    pub fn with_charging_state(mut self, state: ChargingStateEnumType) -> Self {
        self.charging_state = Some(state);
        self
    }

    pub fn with_stopped_reason(mut self, reason: ReasonEnumType) -> Self {
        self.stopped_reason = Some(reason);
        self
    }
}
