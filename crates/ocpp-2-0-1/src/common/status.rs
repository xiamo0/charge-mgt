//! Status and Availability Types (Functional Block F)

use serde::{Deserialize, Serialize};

/// 连接器状态枚举 (StatusNotification)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ConnectorStatusEnumType {
    /// 可用（空闲）
    Available,
    /// 占用
    Occupied,
    /// 已预约
    Reserved,
    /// 不可用
    Unavailable,
    /// 故障
    Faulted,
}

/// 运营状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OperationalStatusEnumType {
    /// 可用
    Operative,
    /// 不可用
    Inoperative,
}

/// 变更可用性状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChangeAvailabilityStatusEnumType {
    /// 已接受
    Accepted,
    /// 已拒绝
    Rejected,
    /// 已计划
    Scheduled,
}

/// 注册状态枚举 (BootNotification)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RegistrationStatusEnumType {
    /// 已接受
    Accepted,
    /// 已拒绝
    Rejected,
    /// 待定
    Pending,
}

/// RequestStartStop 状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RequestStartStopStatusEnumType {
    /// 已接受
    Accepted,
    /// 已拒绝
    Rejected,
}

/// 重启状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ResetStatusEnumType {
    /// 已接受
    Accepted,
    /// 已拒绝
    Rejected,
    /// 已计划
    Scheduled,
}

/// 重启类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ResetEnumType {
    /// 立即重启
    Immediate,
    /// 空闲时重启
    OnIdle,
}

/// 解锁状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UnlockStatusEnumType {
    /// 已解锁
    Unlocked,
    /// 解锁失败
    UnlockFailed,
    /// 正在进行授权事务
    OngoingAuthorizedTransaction,
}

/// 清除缓存状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClearCacheStatusEnumType {
    /// 已接受
    Accepted,
    /// 已拒绝
    Rejected,
}

/// 触发消息状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerMessageStatusEnumType {
    /// 已接受
    Accepted,
    /// 已拒绝
    Rejected,
    /// 未实现
    NotImplemented,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connector_status_enum() {
        let variants = [
            ConnectorStatusEnumType::Available,
            ConnectorStatusEnumType::Occupied,
            ConnectorStatusEnumType::Reserved,
            ConnectorStatusEnumType::Unavailable,
            ConnectorStatusEnumType::Faulted,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ConnectorStatusEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_registration_status_enum() {
        let variants = [
            RegistrationStatusEnumType::Accepted,
            RegistrationStatusEnumType::Rejected,
            RegistrationStatusEnumType::Pending,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: RegistrationStatusEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_request_start_stop_status() {
        let variants = [
            RequestStartStopStatusEnumType::Accepted,
            RequestStartStopStatusEnumType::Rejected,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: RequestStartStopStatusEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_reset_enum() {
        let variants = [
            ResetEnumType::Immediate,
            ResetEnumType::OnIdle,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ResetEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }
}