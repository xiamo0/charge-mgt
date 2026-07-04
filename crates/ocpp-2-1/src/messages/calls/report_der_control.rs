//! ReportDERControl Request (Block R — 2.1 New)
use crate::common::{
    DERCurveGetType, EnterServiceType, FixedVarType, FreqDroopType, GradientType,
    LimitMaxDischargeType,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDERControlRequest {
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve: Option<Vec<DERCurveGetType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enter_service: Option<Vec<EnterServiceType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_var: Option<Vec<FixedVarType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freq_droop: Option<Vec<FreqDroopType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient: Option<Vec<GradientType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_max_discharge: Option<Vec<LimitMaxDischargeType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
}

pub const ACTION: &str = "ReportDERControl";
