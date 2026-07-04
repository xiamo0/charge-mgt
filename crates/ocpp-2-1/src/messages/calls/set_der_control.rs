//! SetDERControl Request (Block R — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::{DERControlEnumType, DERCurveType, EnterServiceType, FixedVarType, FreqDroopType, GradientType, LimitMaxDischargeType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDERControlRequest {
    pub control_id: String,
    pub control_type: DERControlEnumType,
    pub is_default: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve: Option<Vec<DERCurveType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enter_service: Option<EnterServiceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_var: Option<FixedVarType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freq_droop: Option<FreqDroopType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient: Option<GradientType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_max_discharge: Option<LimitMaxDischargeType>,
}

pub const ACTION: &str = "SetDERControl";
