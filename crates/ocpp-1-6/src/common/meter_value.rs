//! 计量值类型

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MeterValue {
    pub timestamp: String,
    pub sampled_value: Vec<SampledValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SampledValue {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ReadingContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ValueFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurand: Option<Measurand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<UnitOfMeasure>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ReadingContext {
    InterruptionBegin,
    InterruptionEnd,
    Other,
    SampleClock,
    SamplePeriodic,
    TransactionBegin,
    TransactionEnd,
    Trigger,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ValueFormat {
    Raw,
    SignedData,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Measurand {
    EnergyActiveImportRegister,
    EnergyReactiveImportRegister,
    EnergyActiveExportRegister,
    EnergyReactiveExportRegister,
    PowerActiveImport,
    PowerReactiveImport,
    PowerActiveExport,
    PowerReactiveExport,
    PowerFactor,
    CurrentImport,
    CurrentExport,
    Voltage,
    Temperature,
    SoC,
    Frequency,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum UnitOfMeasure {
    Wh,
    KWh,
    Varh,
    KVarh,
    W,
    KW,
    Var,
    KVar,
    A,
    V,
    K,
    Celsius,
    Percent,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ReadingContext 测试
    #[test]
    fn test_reading_context_all_variants() {
        let variants = [
            ReadingContext::InterruptionBegin,
            ReadingContext::InterruptionEnd,
            ReadingContext::Other,
            ReadingContext::SampleClock,
            ReadingContext::SamplePeriodic,
            ReadingContext::TransactionBegin,
            ReadingContext::TransactionEnd,
            ReadingContext::Trigger,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ReadingContext = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    // ValueFormat 测试
    #[test]
    fn test_value_format_all_variants() {
        let variants = [ValueFormat::Raw, ValueFormat::SignedData];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ValueFormat = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    // Measurand 测试
    #[test]
    fn test_measurand_all_variants() {
        let variants = [
            Measurand::EnergyActiveImportRegister,
            Measurand::EnergyReactiveImportRegister,
            Measurand::EnergyActiveExportRegister,
            Measurand::EnergyReactiveExportRegister,
            Measurand::PowerActiveImport,
            Measurand::PowerReactiveImport,
            Measurand::PowerActiveExport,
            Measurand::PowerReactiveExport,
            Measurand::PowerFactor,
            Measurand::CurrentImport,
            Measurand::CurrentExport,
            Measurand::Voltage,
            Measurand::Temperature,
            Measurand::SoC,
            Measurand::Frequency,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: Measurand = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    // UnitOfMeasure 测试
    #[test]
    fn test_unit_of_measure_all_variants() {
        let variants = [
            UnitOfMeasure::Wh,
            UnitOfMeasure::KWh,
            UnitOfMeasure::Varh,
            UnitOfMeasure::KVarh,
            UnitOfMeasure::W,
            UnitOfMeasure::KW,
            UnitOfMeasure::Var,
            UnitOfMeasure::KVar,
            UnitOfMeasure::A,
            UnitOfMeasure::V,
            UnitOfMeasure::K,
            UnitOfMeasure::Celsius,
            UnitOfMeasure::Percent,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: UnitOfMeasure = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    // SampledValue 测试
    #[test]
    fn test_sampled_value_minimal() {
        let sv = SampledValue {
            value: "100.0".to_string(),
            context: None,
            format: None,
            measurand: None,
            unit: None,
        };
        let json = serde_json::to_string(&sv).unwrap();
        assert!(!json.contains("context"));
        let de: SampledValue = serde_json::from_str(&json).unwrap();
        assert_eq!(sv.value, de.value);
    }

    #[test]
    fn test_sampled_value_full() {
        let sv = SampledValue {
            value: "50.5".to_string(),
            context: Some(ReadingContext::SamplePeriodic),
            format: Some(ValueFormat::Raw),
            measurand: Some(Measurand::EnergyActiveImportRegister),
            unit: Some(UnitOfMeasure::KWh),
        };
        let json = serde_json::to_string(&sv).unwrap();
        assert!(json.contains("context"));
        assert!(json.contains("format"));
        let de: SampledValue = serde_json::from_str(&json).unwrap();
        assert_eq!(sv.value, de.value);
        assert_eq!(sv.context, de.context);
    }

    #[test]
    fn test_sampled_value_roundtrip() {
        let sv = SampledValue {
            value: "123.456".to_string(),
            context: Some(ReadingContext::TransactionBegin),
            format: Some(ValueFormat::SignedData),
            measurand: Some(Measurand::PowerActiveImport),
            unit: Some(UnitOfMeasure::W),
        };
        let json = serde_json::to_string(&sv).unwrap();
        let de: SampledValue = serde_json::from_str(&json).unwrap();
        assert_eq!(sv, de);
    }

    // MeterValue 测试
    #[test]
    fn test_meter_value_empty() {
        let mv = MeterValue {
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            sampled_value: vec![],
        };
        let json = serde_json::to_string(&mv).unwrap();
        let de: MeterValue = serde_json::from_str(&json).unwrap();
        assert_eq!(mv.timestamp, de.timestamp);
        assert!(de.sampled_value.is_empty());
    }

    #[test]
    fn test_meter_value_with_samples() {
        let mv = MeterValue {
            timestamp: "2024-01-01T12:00:00Z".to_string(),
            sampled_value: vec![
                SampledValue {
                    value: "100.0".to_string(),
                    context: Some(ReadingContext::SamplePeriodic),
                    format: None,
                    measurand: None,
                    unit: None,
                },
                SampledValue {
                    value: "200.0".to_string(),
                    context: Some(ReadingContext::SamplePeriodic),
                    format: None,
                    measurand: None,
                    unit: None,
                },
            ],
        };
        let json = serde_json::to_string(&mv).unwrap();
        let de: MeterValue = serde_json::from_str(&json).unwrap();
        assert_eq!(mv.timestamp, de.timestamp);
        assert_eq!(mv.sampled_value.len(), de.sampled_value.len());
    }

    #[test]
    fn test_meter_value_roundtrip() {
        let mv = MeterValue {
            timestamp: "2024-06-15T10:30:00Z".to_string(),
            sampled_value: vec![
                SampledValue {
                    value: "50.0".to_string(),
                    context: Some(ReadingContext::TransactionBegin),
                    format: Some(ValueFormat::Raw),
                    measurand: Some(Measurand::EnergyActiveImportRegister),
                    unit: Some(UnitOfMeasure::Wh),
                },
            ],
        };
        let json = serde_json::to_string(&mv).unwrap();
        let de: MeterValue = serde_json::from_str(&json).unwrap();
        assert_eq!(mv.timestamp, de.timestamp);
        assert_eq!(mv.sampled_value, de.sampled_value);
    }
}