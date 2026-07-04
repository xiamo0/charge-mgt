//! OCPP 2.1 Bidirectional / V2X types (Functional Block Q)

use serde::{Deserialize, Serialize};

/// 能量传输模式枚举 (含双向)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EnergyTransferModeEnumType {
    #[serde(rename = "AC_single_phase")]
    ACSinglePhase,
    #[serde(rename = "AC_three_phase")]
    ACThreePhase,
    DC,
    #[serde(rename = "AC_BPT")]
    ACBPT,
    #[serde(rename = "DC_BPT")]
    DCBPT,
}

/// 通知允许的能量传输状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum NotifyAllowedEnergyTransferStatusEnumType {
    Accepted,
    Rejected,
}
