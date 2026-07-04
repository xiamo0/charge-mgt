//! Meter Value Types (Functional Block J)

use serde::{Deserialize, Serialize};

/// 测量量枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MeasurandEnumType {
    EnergyActiveImportRegister,
    EnergyActiveExportRegister,
    EnergyReactiveImportRegister,
    EnergyReactiveExportRegister,
    EnergyActiveImportInterval,
    EnergyActiveExportInterval,
    EnergyActiveNet,
    EnergyReactiveImportInterval,
    EnergyReactiveExportInterval,
    EnergyReactiveNet,
    EnergyApparentNet,
    EnergyApparentImport,
    EnergyApparentExport,
    Frequency,
    PowerActiveImport,
    PowerActiveExport,
    PowerFactor,
    PowerOffered,
    PowerReactiveImport,
    PowerReactiveExport,
    SoC,
    CurrentImport,
    CurrentExport,
    CurrentOffered,
    Voltage,
}

/// 读数上下文枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ReadingContextEnumType {
    InterruptionBegin,
    InterruptionEnd,
    Other,
    SampleClock,
    SamplePeriodic,
    TransactionBegin,
    TransactionEnd,
    Trigger,
}

/// 相位枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PhaseEnumType {
    L1,
    L2,
    L3,
    N,
    #[serde(rename = "L1-N")]
    L1N,
    #[serde(rename = "L2-N")]
    L2N,
    #[serde(rename = "L3-N")]
    L3N,
    #[serde(rename = "L1-L2")]
    L1L2,
    #[serde(rename = "L2-L3")]
    L2L3,
    #[serde(rename = "L3-L1")]
    L3L1,
}

/// 位置枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LocationEnumType {
    Body,
    Cable,
    EV,
    Inlet,
    Outlet,
}

/// 计量单位类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitOfMeasureType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<i32>,
}

impl Default for UnitOfMeasureType {
    fn default() -> Self {
        Self {
            unit: Some("Wh".to_string()),
            multiplier: Some(0),
        }
    }
}

/// 签名计量值类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedMeterValueType {
    pub signed_meter_data: String,
    pub signing_method: String,
    pub encoding_method: String,
    pub public_key: String,
}

/// 采样值类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampledValueType {
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ReadingContextEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurand: Option<MeasurandEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<PhaseEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_meter_value: Option<SignedMeterValueType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<UnitOfMeasureType>,
}

impl SampledValueType {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            context: None,
            measurand: None,
            phase: None,
            location: None,
            signed_meter_value: None,
            unit_of_measure: None,
        }
    }

    pub fn energy(value: f64) -> Self {
        Self {
            value,
            context: None,
            measurand: Some(MeasurandEnumType::EnergyActiveImportRegister),
            phase: None,
            location: None,
            signed_meter_value: None,
            unit_of_measure: Some(UnitOfMeasureType::default()),
        }
    }
}

/// 计量值类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterValueType {
    pub timestamp: String,
    pub sampled_value: Vec<SampledValueType>,
}

impl MeterValueType {
    pub fn new(timestamp: impl Into<String>, sampled_values: Vec<SampledValueType>) -> Self {
        Self {
            timestamp: timestamp.into(),
            sampled_value: sampled_values,
        }
    }
}
