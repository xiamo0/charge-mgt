//! 配置类型

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigurationKey {
    pub key: String,
    pub value: String,
    #[serde(rename = "readonly")]
    pub readonly: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizationData {
    #[serde(rename = "idTag")]
    pub id_tag: String,
    #[serde(rename = "idTagInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<crate::common::id_tag::IdTagInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalAuthorizeList {
    #[serde(rename = "localAuthorizationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_authorization_list: Option<Vec<AuthorizationData>>,
    #[serde(rename = "listVersion")]
    pub list_version: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChargingSchedule {
    pub duration: Option<i32>,
    #[serde(rename = "startPeriod")]
    pub start_period: Option<i32>,
    #[serde(rename = "chargingRateUnit")]
    pub charging_rate_unit: String,
    #[serde(rename = "chargingSchedulePeriod")]
    pub charging_schedule_period: Vec<ChargingSchedulePeriod>,
    #[serde(rename = "minChargingRate")]
    pub min_charging_rate: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChargingSchedulePeriod {
    #[serde(rename = "startPeriod")]
    pub start_period: i32,
    pub limit: f64,
    #[serde(rename = "numberPhases")]
    pub number_phases: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChargingProfile {
    #[serde(rename = "chargingProfileId")]
    pub charging_profile_id: i32,
    #[serde(rename = "stackLevel")]
    pub stack_level: i32,
    #[serde(rename = "chargingProfilePurpose")]
    pub charging_profile_purpose: String,
    #[serde(rename = "chargingProfileKind")]
    pub charging_profile_kind: String,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    #[serde(rename = " ChargingSchedule")]
    pub charging_schedule: Option<ChargingSchedule>,
    #[serde(rename = "recurrencyKind")]
    pub recurrency_kind: Option<String>,
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<i32>,
}
