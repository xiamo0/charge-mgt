//! Transaction Types (Functional Block E)

use serde::{Deserialize, Serialize};

/// 事务事件类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TransactionEventEnumType {
    /// 事务开始
    Started,
    /// 事务更新
    Updated,
    /// 事务结束
    Ended,
}

/// 触发原因枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerReasonEnumType {
    /// 授权通过
    Authorized,
    /// 插入线缆
    CablePluggedIn,
    /// 充电速率变化
    ChargingRateChanged,
    /// 充电状态变化
    ChargingStateChanged,
    /// 授权撤销
    Deauthorized,
    /// 达到能量限制
    EnergyLimitReached,
    /// EV通信丢失
    EVCommunicationLost,
    /// EV连接超时
    EVConnectTimeout,
    /// 时钟对齐采样
    MeterValueClock,
    /// 周期采样
    MeterValuePeriodic,
    /// 达到时间限制
    TimeLimitReached,
    /// 被触发
    Trigger,
    /// 解锁命令
    UnlockCommand,
    /// 停止授权
    StopAuthorized,
    /// EV离开
    EVDeparted,
    /// 检测到EV
    EVDetected,
    /// 远程停止
    RemoteStop,
    /// 远程启动
    RemoteStart,
    /// 异常状况
    AbnormalCondition,
    /// 收到签名数据
    SignedDataReceived,
    /// 复位命令
    ResetCommand,
}

/// 充电状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingStateEnumType {
    /// 充电中
    Charging,
    /// EV已连接
    EVConnected,
    /// EV侧暂停
    SuspendedEV,
    /// EVSE侧暂停
    SuspendedEVSE,
    /// 空闲
    Idle,
}

/// 停止原因枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ReasonEnumType {
    /// 授权撤销
    DeAuthorized,
    /// 急停
    EmergencyStop,
    /// 能量限制
    EnergyLimitReached,
    /// EV断开
    EVDisconnected,
    /// 接地故障
    GroundFault,
    /// 立即复位
    ImmediateReset,
    /// 本地停止
    Local,
    /// 本地余额不足
    LocalOutOfCredit,
    /// 主卡
    MasterPass,
    /// 其他
    Other,
    /// 过流故障
    OvercurrentFault,
    /// 掉电
    PowerLoss,
    /// 电能质量
    PowerQuality,
    /// 重启
    Reboot,
    /// 远程停止
    Remote,
    /// SoC限制
    SOCLimitReached,
    /// EV停止
    StoppedByEV,
    /// 时间限制
    TimeLimitReached,
    /// 超时
    Timeout,
}

/// 事务信息类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionType {
    /// 事务ID (max 36 chars)
    pub transaction_id: String,
    /// 充电状态 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_state: Option<ChargingStateEnumType>,
    /// 实际充电时间(秒) (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_spent_charging: Option<i32>,
    /// 停止原因 (可选, 仅Ended时)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_reason: Option<ReasonEnumType>,
    /// 关联的远程启动ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_start_id: Option<i32>,
}

impl TransactionType {
    /// 创建新的事务信息
    pub fn new(transaction_id: impl Into<String>) -> Self {
        Self {
            transaction_id: transaction_id.into(),
            charging_state: None,
            time_spent_charging: None,
            stopped_reason: None,
            remote_start_id: None,
        }
    }

    /// 设置充电状态
    pub fn with_charging_state(mut self, state: ChargingStateEnumType) -> Self {
        self.charging_state = Some(state);
        self
    }

    /// 设置充电时间
    pub fn with_time_spent_charging(mut self, seconds: i32) -> Self {
        self.time_spent_charging = Some(seconds);
        self
    }

    /// 设置停止原因
    pub fn with_stopped_reason(mut self, reason: ReasonEnumType) -> Self {
        self.stopped_reason = Some(reason);
        self
    }

    /// 设置远程启动ID
    pub fn with_remote_start_id(mut self, id: i32) -> Self {
        self.remote_start_id = Some(id);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_event_enum() {
        let variants = [
            TransactionEventEnumType::Started,
            TransactionEventEnumType::Updated,
            TransactionEventEnumType::Ended,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: TransactionEventEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_trigger_reason_enum() {
        let variants = [
            TriggerReasonEnumType::Authorized,
            TriggerReasonEnumType::CablePluggedIn,
            TriggerReasonEnumType::RemoteStart,
            TriggerReasonEnumType::RemoteStop,
            TriggerReasonEnumType::MeterValuePeriodic,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: TriggerReasonEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_charging_state_enum() {
        let variants = [
            ChargingStateEnumType::Charging,
            ChargingStateEnumType::EVConnected,
            ChargingStateEnumType::SuspendedEV,
            ChargingStateEnumType::SuspendedEVSE,
            ChargingStateEnumType::Idle,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ChargingStateEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_reason_enum() {
        let variants = [
            ReasonEnumType::DeAuthorized,
            ReasonEnumType::EmergencyStop,
            ReasonEnumType::EVDisconnected,
            ReasonEnumType::Remote,
            ReasonEnumType::Local,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ReasonEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_transaction_type() {
        let trans =
            TransactionType::new("TX-001").with_charging_state(ChargingStateEnumType::Charging);
        assert_eq!(trans.transaction_id, "TX-001");
        assert_eq!(trans.charging_state, Some(ChargingStateEnumType::Charging));

        let json = serde_json::to_string(&trans).unwrap();
        let de: TransactionType = serde_json::from_str(&json).unwrap();
        assert_eq!(trans, de);
    }
}
